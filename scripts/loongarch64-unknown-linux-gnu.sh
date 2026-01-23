#!/usr/bin/env bash

set -e

echo "Use the last snapshot without dependency issues for now..."
rm -f /etc/apt/sources.list.d/*
cat << EOF | tee /etc/apt/sources.list.d/debian-ports.list
deb http://snapshot.debian.org/archive/debian-ports/20260108T013037Z/ sid main
EOF
( echo 'quiet "true";'; \
  echo 'APT::Get::Assume-Yes "true";'; \
  echo 'APT::Install-Recommends "false";'; \
  echo 'Acquire::Check-Valid-Until "false";'; \
  echo 'Acquire::Retries "5";'; \
) > /etc/apt/apt.conf.d/99snapshot-repos

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

rustup target add loongarch64-unknown-linux-gnu

export CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_LINKER=gcc

yarn build --target loongarch64-unknown-linux-gnu
