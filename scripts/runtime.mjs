import { existsSync } from 'node:fs';
import { resolve, delimiter } from 'node:path';
import { homedir } from 'node:os';
export function rustEnv({ release = false } = {}) {
  const cargo = resolve('.tools/cargo');
  const rustup = resolve('.tools/rustup');
  const local = existsSync(cargo + '/bin/' + (process.platform === 'win32' ? 'cargo.exe' : 'cargo'));
  const env = local ? { ...process.env, CARGO_HOME: cargo, RUSTUP_HOME: rustup, PATH: cargo + '/bin' + delimiter + process.env.PATH } : { ...process.env };
  if (release) {
    // Encoded arguments preserve paths containing spaces. Remove build-machine paths
    // from embedded panic messages/debug locations before publishing native binaries.
    const flags = env.CARGO_ENCODED_RUSTFLAGS?.split('\x1f') || env.RUSTFLAGS?.split(/\s+/).filter(Boolean) || [];
    flags.push(`--remap-path-prefix=${resolve('.')}=/build/promptbeacon`, `--remap-path-prefix=${homedir()}=/build/home`);
    env.CARGO_ENCODED_RUSTFLAGS = flags.join('\x1f');
    delete env.RUSTFLAGS;
  }
  return env;
}
