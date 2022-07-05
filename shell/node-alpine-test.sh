#!/bin/sh

set -e

apk update
apk add libx11 libxrandr dbus

yarn test