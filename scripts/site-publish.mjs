import { spawnSync } from 'node:child_process';
import { cp, lstat, mkdtemp, readdir, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const remote = 'https://github.com/Tengify/promptbeacon.git';
const source = fileURLToPath(new URL('../website/dist/', import.meta.url));
const expected = new Set(['.nojekyll', 'index.html', 'style.css', 'main.js', 'releases.js', 'assets/beacon.svg', 'assets/sora-bold.ttf', 'assets/Sora-OFL.txt']);

async function inspect(directory, prefix = '') {
  const files = [];
  for (const name of await readdir(directory)) {
    const path = join(directory, name);
    const relative = `${prefix}${name}`;
    const entry = await lstat(path);
    if (entry.isSymbolicLink()) throw new Error(`Refusing to publish a symlink: ${relative}`);
    if (entry.isDirectory()) files.push(...await inspect(path, `${relative}/`));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`Unsupported file: ${relative}`);
  }
  return files;
}

const files = await inspect(source);
if (files.length !== expected.size || files.some(file => !expected.has(file))) {
  throw new Error('The built site contains missing or unexpected files. Run npm run site:build and inspect website/dist.');
}

function git(args, cwd, accepted = [0]) {
  const result = spawnSync('git', args, { cwd, encoding: 'utf8', env: { ...process.env, GIT_TERMINAL_PROMPT: '0' } });
  if (result.error) throw result.error;
  if (!accepted.includes(result.status)) throw new Error(result.stderr || `git ${args[0]} failed`);
  return result;
}

const branch = git(['ls-remote', '--exit-code', '--heads', remote, 'gh-pages'], source, [0, 2]);
const checkout = await mkdtemp(join(tmpdir(), 'promptbeacon-pages-'));
try {
  git(['init', '--initial-branch=gh-pages'], checkout);
  git(['config', 'user.name', 'Tengify'], checkout);
  git(['config', 'user.email', 'noreply@users.noreply.github.com'], checkout);
  git(['remote', 'add', 'origin', remote], checkout);
  if (branch.status === 0) {
    git(['fetch', '--depth=1', 'origin', 'gh-pages'], checkout);
    git(['checkout', '-B', 'gh-pages', 'FETCH_HEAD'], checkout);
    for (const name of await readdir(checkout)) {
      if (name !== '.git') await rm(join(checkout, name), { recursive: true, force: true });
    }
  }
  for (const file of files) await cp(join(source, file), join(checkout, file));
  git(['add', '--all'], checkout);
  const changed = git(['diff', '--cached', '--quiet'], checkout, [0, 1]);
  if (changed.status === 0) {
    console.log('Published branch already matches the local build.');
  } else {
    git(['commit', '-m', 'Publish locally built PromptBeacon landing page'], checkout);
    git(['push', 'origin', 'gh-pages'], checkout);
    console.log('Published public landing assets to gh-pages. Main checkout unchanged.');
  }
} finally {
  await rm(checkout, { recursive: true, force: true });
}
