// Optional local Windows cross-build. Native Windows builds use npm run app:build.
import { cp, mkdir, mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawn } from 'node:child_process';
import { rustEnv } from './runtime.mjs';
const root = resolve('.');
const stage = await mkdtemp(join(tmpdir(), 'promptbeacon-windows-'));
await mkdir(join(stage, 'src-tauri'));
for (const name of ['Cargo.toml', 'Cargo.lock', 'build.rs', 'tauri.conf.json', 'src', 'icons', 'capabilities']) {
  await cp(join(root, 'src-tauri', name), join(stage, 'src-tauri', name), { recursive: true });
}
await cp(join(root, 'dist'), join(stage, 'dist'), { recursive: true });
const env = rustEnv();
// cargo-xwin's target rustflags are space-separated. Build in a temporary path
// without spaces and use parent-prefix remaps rather than a personal source path.
delete env.CARGO_ENCODED_RUSTFLAGS;
env.RUSTFLAGS = `--remap-path-prefix=/Volumes=/build/volumes --remap-path-prefix=/Users=/build/users --remap-path-prefix=${stage}=/build/promptbeacon`;
env.XWIN_CACHE_DIR ||= join(tmpdir(), 'promptbeacon-xwin-cache');
const child = spawn('cargo', ['xwin', 'build', '--manifest-path', join(stage, 'src-tauri/Cargo.toml'), '--target', 'x86_64-pc-windows-msvc', '--release', '--locked', '-j', '4'], { cwd: stage, stdio: 'inherit', env });
const code = await new Promise((res, rej) => { child.once('error', rej); child.once('exit', code => res(code ?? 1)); });
if (code === 0) {
  await mkdir(join(root, 'artifacts/release'), { recursive: true });
  await cp(join(stage, 'src-tauri/target/x86_64-pc-windows-msvc/release/promptbeacon.exe'), join(root, 'artifacts/release/PromptBeacon.exe'));
}
await rm(stage, { recursive: true, force: true });
process.exitCode = code;
