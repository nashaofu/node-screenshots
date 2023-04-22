#!/bin/ash

DISPLAY="${DISPLAY:-:0}"
DISPLAY_WIDTH="${DISPLAY_WIDTH:-1280}"
DISPLAY_HEIGHT="${DISPLAY_HEIGHT:-720}"

# 后台运行程序
startInBackgroundIfNotRunning() {
  echo "Starting $1."
  if ! pidof $1 >/dev/null; then
    $@ &
    while ! pidof $1 >/dev/null; do
      echo "Waiting $1 start"
      sleep 1
    done
    echo "$1 started."
  else
    echo "$1 is already running."
  fi
}

startInBackgroundIfNotRunning Xvfb ${DISPLAY} -screen 0 ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}x24 -dpi 96 -listen tcp -ac
startInBackgroundIfNotRunning fluxbox -display ${DISPLAY}

echo $@

$@
