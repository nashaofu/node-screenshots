#!/bin/sh

set -e

export DISPLAY="${DISPLAY:-:1}"
export VNC_PORT="${VNC_PORT:-5900}"
export NOVNC_PORT="${NOVNC_PORT:-6080}"
export DISPLAY_WIDTH="${DISPLAY_WIDTH:-1280}"
export DISPLAY_HEIGHT="${DISPLAY_HEIGHT:-720}"

apt-get update
apt-get -y install --no-install-recommends \
    fluxbox \
    xvfb \
    x11vnc \
    novnc \
    libxcb1 \
    libxrandr2 \
    libdbus-1-3

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

echo "Displaying on $DISPLAY."

DISPLAY=$DISPLAY startfluxbox &
sleep 5
startInBackgroundIfNotRunning Xvfb ${DISPLAY} -screen 0 ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}x24 -dpi 96 -listen tcp -ac
startInBackgroundIfNotRunning x11vnc -display ${DISPLAY} -ncache 10 -rfbport ${VNC_PORT} -forever
startInBackgroundIfNotRunning websockify --web /usr/share/novnc ${NOVNC_PORT} localhost:${VNC_PORT}

yarn test
