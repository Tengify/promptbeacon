#!/bin/sh
set -eu
mkdir -p /build/src-tauri
cp /source/src-tauri/Cargo.toml /source/src-tauri/Cargo.lock /source/src-tauri/build.rs /source/src-tauri/tauri.conf.json /build/src-tauri/
cp -R /source/src-tauri/src /source/src-tauri/icons /source/src-tauri/capabilities /build/src-tauri/
cp -R /source/dist /build/dist
cargo build --manifest-path /build/src-tauri/Cargo.toml --release --locked --target x86_64-unknown-linux-gnu -j 2
cp /build/src-tauri/target/x86_64-unknown-linux-gnu/release/promptbeacon /artifacts/promptbeacon-linux-x86_64
