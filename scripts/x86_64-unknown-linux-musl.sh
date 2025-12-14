#!/usr/bin/env bash

set -e

echo "Updating package lists and installing dependencies..."
apk update
apk add \
  curl \
  build-base \
  pkgconf \
  clang-dev \
  libxcb-dev \
  libxrandr-dev \
  dbus-dev \
  pipewire-dev

echo "Installing rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

rustup target add x86_64-unknown-linux-musl

yarn build --target x86_64-unknown-linux-musl
