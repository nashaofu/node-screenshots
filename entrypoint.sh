#!/usr/bin/env bash

echo "Starting..."
yarn install

echo "Build..."
yarn build --target aarch64-unknown-linux-musl
