#!/usr/bin/env bash

set -e
apk update
apk add curl build-base pkgconf clang-dev libxcb-dev libxrandr-dev dbus-dev pipewire-dev

curl -fsSL https://fnm.vercel.app/install | bash
source $HOME/.bashrc
fnm install 16
fnm use 16
npm install -g yarn

yarn build --target x86_64-unknown-linux-musl
