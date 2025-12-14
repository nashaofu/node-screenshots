#!/usr/bin/env bash

set -e
apk update
apk add curl pkgconf clang-dev libxcb-dev libxrandr-dev dbus-dev pipewire-dev

curl -fsSL https://fnm.vercel.app/install | ash
source $HOME/.bashrc
fnm install 24
fnm use 24
npm install -g yarn

yarn build --target x86_64-unknown-linux-musl
