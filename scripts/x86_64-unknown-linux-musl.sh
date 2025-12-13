#!/usr/bin/env bash

set -e
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libclang-dev libxcb1-dev libxrandr-dev libdbus-1-dev libpipewire-0.3-dev libwayland-dev libegl-dev
yarn build --target x86_64-unknown-linux-musl -x
