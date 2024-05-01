ARG VERSION="16"

FROM node:${VERSION}-slim

# Setup environment variables
ENV DEBIAN_FRONTEND=noninteractive \
  DISPLAY=":0" \
  DISPLAY_WIDTH="1280" \
  DISPLAY_HEIGHT="720"

RUN apt-get update && \
  apt-get -y install --no-install-recommends procps xvfb xfce4 dbus-x11 libxcb1 libxrandr2 && \
  apt-get autoclean -y && \
  apt-get autoremove -y  && \
  rm -rf /var/lib/apt/lists/*

COPY debian-init.sh /usr/local/share/debian-init.sh

ENTRYPOINT [ "/usr/local/share/debian-init.sh" ]

EXPOSE 6080
