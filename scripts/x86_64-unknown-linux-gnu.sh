#!/usr/bin/env bash

set -e
apt-get update
apt-get install -y curl build-essential pkg-config libclang-dev libxcb1-dev libxrandr-dev libdbus-1-dev libpipewire-0.3-dev

curl -fsSL https://fnm.vercel.app/install | bash
source $HOME/.bashrc
fnm install 16
fnm use 16
npm install -g yarn

yarn build --target x86_64-unknown-linux-gnu
