// Only publish links for artifacts that were built locally and uploaded to this release.
export const release = {
  version: '0.2.0',
  base: 'https://github.com/Tengify/promptbeacon/releases/download/v0.2.0/',
  assets: {
    macos: 'PromptBeacon_0.2.0_macOS_universal.zip',
    windows: 'PromptBeacon_0.2.0_windows_x64.zip',
    linux: 'promptbeacon_0.2.0_amd64.deb',
  },
};
