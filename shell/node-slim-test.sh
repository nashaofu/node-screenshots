#!/bin/sh

set -e

apt-get update
apt-get install libx11-6 libxrandr2 libdbus-1-3 -y

yarn test
