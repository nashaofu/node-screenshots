#!/bin/ash

DISPLAY="${DISPLAY:-:0}"
DISPLAY_WIDTH="${DISPLAY_WIDTH:-1280}"
DISPLAY_HEIGHT="${DISPLAY_HEIGHT:-720}"

# Execute the command it not already running
startInBackgroundIfNotRunning() {
  log "Starting $1."
  echo -e "\n** $(date) **" | sudoIf tee -a /tmp/$1.log >/dev/null
  if ! pgrep -x $1 >/dev/null; then
    keepRunningInBackground "$@"
    while ! pgrep -x $1 >/dev/null; do
      sleep 1
    done
    log "$1 started."
  else
    echo "$1 is already running." | sudoIf tee -a /tmp/$1.log >/dev/null
    log "$1 is already running."
  fi
}

# Keep command running in background
keepRunningInBackground() {
  (
    $2 ash -c "while :; do echo [\$(date)] Process started.; $3; echo [\$(date)] Process exited!; sleep 5; done 2>&1" | sudoIf tee -a /tmp/$1.log >/dev/null &
    echo "$!" | sudoIf tee /tmp/$1.pid >/dev/null
  )
}

# Use sudo to run as root when required
sudoIf() {
  if [ "$(id -u)" -ne 0 ]; then
    sudo "$@"
  else
    "$@"
  fi
}

# Log messages
log() {
  echo -e "[$(date)] $@" | sudoIf tee -a /tmp/container-init.log >/dev/null
}

log "** SCRIPT START **"

startInBackgroundIfNotRunning "Xvfb" sudoIf "Xvfb ${DISPLAY} -screen 0 ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}x16 -listen tcp -ac"
startInBackgroundIfNotRunning "xfdesktop" sudoIf "startxfce4"

log "Executing \"$@\"."
exec "$@"
log "** SCRIPT EXIT **"
