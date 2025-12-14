#!/usr/bin/env bash

set -e

echo "Updating package lists and installing dependencies..."
apt-get update
apt-get install -y \
  curl \
  build-essential \
  pkg-config \
  libclang-dev \
  libxcb1-dev \
  libxrandr-dev \
  libdbus-1-dev \
  libpipewire-0.3-dev

echo "Installing rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

rustup target add x86_64-unknown-linux-gnu

yarn build --target x86_64-unknown-linux-gnu
