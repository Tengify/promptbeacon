FROM rust:1.99-bookworm
RUN apt-get update && apt-get install -y --no-install-recommends libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev patchelf ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /build
