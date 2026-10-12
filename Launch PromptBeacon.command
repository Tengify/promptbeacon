#!/bin/zsh
project_dir="${0:A:h}"
for app_path in "$project_dir/src-tauri/target/universal-apple-darwin/release/bundle/macos/PromptBeacon.app" "$project_dir/src-tauri/target/release/bundle/macos/PromptBeacon.app" "$project_dir/artifacts/macOS/PromptBeacon.app"; do
  if [[ -d "$app_path" ]]; then
    open "$app_path"
    exit
  fi
done
print 'Build PromptBeacon first with npm run app:build, or install a release.'
exit 1
