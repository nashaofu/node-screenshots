ARG VERSION="18"

FROM node:${VERSION}-alpine

# Setup environment variables
ENV DISPLAY=":0" \
  VNC_PORT="5900" \
  NOVNC_PORT="6080" \
  DISPLAY_WIDTH="1280" \
  DISPLAY_HEIGHT="720"

COPY alpine-init.sh /usr/local/share/alpine-init.sh

RUN echo "http://dl-3.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories

RUN apk update && \
  apk add xvfb fluxbox libxcb libxrandr dbus

ENTRYPOINT [ "/usr/local/share/alpine-init.sh" ]

EXPOSE 6080
