#!/usr/bin/env bash

apt-get update
apt-get install libxcb1-dev -y

apt-get update && apt-get install -y --no-install-recommends \
  gcc libc6-dev ca-certificates \
  gcc-aarch64-linux-gnu libc6-dev-arm64-cross qemu-user

export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"

rustup target add aarch64-unknown-linux-gnu
rustup toolchain install stable-aarch64-unknown-linux-gnu
yarn build --target aarch64-unknown-linux-gnu
aarch64-unknown-linux-gnu-strip *.node
