#!/bin/sh
set -eu
version=0.2.0
root=/build/package
mkdir -p "$root/DEBIAN" "$root/usr/bin" "$root/usr/share/applications" "$root/usr/share/icons/hicolor/128x128/apps" "$root/usr/share/doc/promptbeacon"
install -m 755 /artifacts/promptbeacon-linux-x86_64 "$root/usr/bin/promptbeacon"
install -m 644 /source/src-tauri/icons/128x128.png "$root/usr/share/icons/hicolor/128x128/apps/promptbeacon.png"
cp /source/LICENSE "$root/usr/share/doc/promptbeacon/copyright"
gzip -9 -c /source/THIRD_PARTY_NOTICES.md > "$root/usr/share/doc/promptbeacon/THIRD_PARTY_NOTICES.md.gz"
cat > "$root/usr/share/applications/promptbeacon.desktop" <<'DESKTOP'
[Desktop Entry]
Type=Application
Name=PromptBeacon
Comment=Know your model before you prompt
Exec=promptbeacon
Icon=promptbeacon
Terminal=false
Categories=Development;Utility;
DESKTOP
size=$(du -sk "$root/usr" | cut -f1)
cat > "$root/DEBIAN/control" <<CONTROL
Package: promptbeacon
Version: $version
Architecture: amd64
Maintainer: PromptBeacon contributors <noreply@users.noreply.github.com>
Installed-Size: $size
Depends: libc6 (>= 2.36), libwebkit2gtk-4.1-0, libgtk-3-0, libayatana-appindicator3-1, libxdo3, libdbus-1-3
Section: devel
Priority: optional
Homepage: https://github.com/Tengify/promptbeacon
Description: Model and effort indicator for coding sessions
 A local Rust and Tauri utility with a click-through model color and effort
 beacon. Preview release; native desktop accessibility depends on app and OS.
CONTROL
chmod -R go-w "$root"
dpkg-deb --root-owner-group --build "$root" "/artifacts/promptbeacon_${version}_amd64.deb"
dpkg-deb --info "/artifacts/promptbeacon_${version}_amd64.deb"
