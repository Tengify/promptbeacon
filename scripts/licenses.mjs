import { spawnSync } from 'node:child_process';
import { readFile, writeFile, readdir } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { dirname, join } from 'node:path';
import { rustEnv } from './runtime.mjs';
const result = spawnSync('cargo', ['metadata', '--manifest-path', 'src-tauri/Cargo.toml', '--format-version', '1', '--locked'], { env: rustEnv(), encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 });
if (result.status !== 0) throw new Error('Cargo metadata failed. Install locked dependencies before generating notices.');
const metadata = JSON.parse(result.stdout);
const notices = new Map();
function register(text) {
  text = text.replace(/\r\n/g, '\n').split('\n').map(line => line.trimEnd()).join('\n').trim();
  const hash = createHash('sha256').update(text).digest('hex');
  if (!notices.has(hash)) notices.set(hash, { id: `notice-${notices.size + 1}`, text });
  return notices.get(hash).id;
}
const sections = ['# Third-party notices\n\nPromptBeacon itself is MIT licensed. This inventory includes the locked Rust dependency graph (including build and optional platform dependencies), bundled Tauri JavaScript API, Rust runtime licenses, and optional landing-page font. Dependency authors retain their licenses. System webviews/libraries are supplied by the OS. Identical license texts are shared by reference to keep distribution small.\n'];
let missing = 0;
for (const pkg of metadata.packages.filter(p => p.source).sort((a,b) => a.name.localeCompare(b.name) || a.version.localeCompare(b.version))) {
  const dir = dirname(pkg.manifest_path);
  sections.push(`\n## ${pkg.name} ${pkg.version}\n\nLicense: ${pkg.license || 'See original source'}. Source: https://crates.io/crates/${pkg.name}/${pkg.version}\n`);
  const names = (await readdir(dir)).filter(n => /^(license|copying|notice)(?:$|[._-])/i.test(n));
  let count = 0;
  for (const name of names) {
    try { const id = register(await readFile(join(dir,name),'utf8')); sections.push(`\n${name}: [${id}](#${id}).\n`); count++; } catch { /* Directory/non-text notice; original source is linked. */ }
  }
  if (!count) { sections.push('\nLicense text: see the linked original crate source.\n'); missing++; }
}
sections.push('\n## Tauri JavaScript API\n');
for (const name of ['LICENSE-MIT','LICENSE-APACHE-2.0']) { const id=register(await readFile(join('node_modules/@tauri-apps/api',name),'utf8')); sections.push(`\n${name}: [${id}](#${id}).\n`); }
sections.push(`\n## Sora font\n\nUsed by the optional landing page. License: [${register(await readFile('website/assets/Sora-OFL.txt','utf8'))}](#${register(await readFile('website/assets/Sora-OFL.txt','utf8'))}).\n`);
const rustup = rustEnv().RUSTUP_HOME;
if (rustup) {
  const stable = (await readdir(join(rustup,'toolchains'))).find(n => n.startsWith('stable-'));
  if (stable) {
    const dir = join(rustup,'toolchains',stable,'share/doc/rust/licenses');
    for (const name of (await readdir(dir)).filter(n => /mit|apache/i.test(n))) { const id=register(await readFile(join(dir,name),'utf8')); sections.push(`\n## Rust runtime — ${name}\n\n[${id}](#${id}).\n`); }
  }
}
for (const {id,text} of notices.values()) sections.push(`\n## ${id}\n\n\`\`\`text\n${text}\n\`\`\`\n`);
await writeFile('THIRD_PARTY_NOTICES.md',sections.join(''));
console.log(`Generated ${notices.size} distinct notices for ${metadata.packages.filter(p=>p.source).length} locked crates; ${missing} use original-source license links.`);
