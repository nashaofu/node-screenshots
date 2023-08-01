ARG VERSION="18"

FROM node:${VERSION}-slim

# Setup environment variables
ENV DEBIAN_FRONTEND=noninteractive \
    DISPLAY=":0" \
    DISPLAY_WIDTH="1280" \
    DISPLAY_HEIGHT="720"

COPY debian-init.sh /usr/local/share/debian-init.sh

RUN apt-get update && \
    apt-get -y install --no-install-recommends xvfb libxcb1 libxrandr2 && \
    apt-get autoclean -y && \
    apt-get autoremove -y  && \
    rm -rf /var/lib/apt/lists/*

ENTRYPOINT [ "/usr/local/share/debian-init.sh" ]

EXPOSE 6080
