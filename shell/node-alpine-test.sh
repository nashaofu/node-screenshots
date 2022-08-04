#!/bin/ash

set -e

DISPLAY="${DISPLAY:-:1}"
VNC_PORT="${VNC_PORT:-5900}"
NOVNC_PORT="${NOVNC_PORT:-6080}"
DISPLAY_WIDTH="${DISPLAY_WIDTH:-1280}"
DISPLAY_HEIGHT="${DISPLAY_HEIGHT:-720}"

echo "http://dl-3.alpinelinux.org/alpine/edge/testing" >>/etc/apk/repositories

apk update
apk add fluxbox xvfb x11vnc novnc libxcb libxrandr dbus

# 后台运行程序
startInBackgroundIfNotRunning() {
    echo "Starting $1."
    if ! pidof $1 >/dev/null; then
        $@ &
        sleep 3
        while ! pidof $1 >/dev/null; do
            sleep 1
        done
        echo "$1 started."
    else
        echo "$1 is already running."
    fi
}

DISPLAY=$DISPLAY startfluxbox &
sleep 5
startInBackgroundIfNotRunning Xvfb ${DISPLAY} -screen 0 ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}x24 -dpi 96 -listen tcp -ac
startInBackgroundIfNotRunning x11vnc -display ${DISPLAY} -rfbport ${VNC_PORT} -forever
startInBackgroundIfNotRunning websockify --web /usr/share/novnc ${NOVNC_PORT} localhost:${VNC_PORT}

yarn test
