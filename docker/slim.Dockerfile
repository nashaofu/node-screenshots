ARG VERSION="16"

FROM node:${VERSION}-slim

# Setup environment variables
ENV DEBIAN_FRONTEND=noninteractive \
    DISPLAY=":0" \
    VNC_PORT="5900" \
    NOVNC_PORT="6080" \
    DISPLAY_WIDTH="1280" \
    DISPLAY_HEIGHT="720"

COPY slim-init.sh /usr/local/share/slim.sh

RUN apt-get update && \
    apt-get -y install --no-install-recommends \
    xvfb \
    fluxbox \
    libxcb1 \
    libxrandr2 \
    libdbus-1-3

ENTRYPOINT [ "/usr/local/share/slim.sh" ]

EXPOSE 6080