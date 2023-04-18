# [Choice] Node.js version (use -bullseye variants on local arm64/Apple Silicon): 18, 16, 14, 18-bullseye, 16-bullseye, 14-bullseye, 18-buster, 16-buster, 14-buster
ARG VARIANT=16-bullseye
FROM mcr.microsoft.com/vscode/devcontainers/typescript-node:0-${VARIANT}

# Setup environment variables
ENV DEBIAN_FRONTEND=noninteractive \
  DISPLAY=":0" \
  VNC_PORT="5900" \
  NOVNC_PORT="6080" \
  DISPLAY_WIDTH="1280" \
  DISPLAY_HEIGHT="720"

COPY build.sh /tmp/build.sh
COPY docker-init.sh /usr/local/share/docker-init.sh

RUN bash /tmp/build.sh && \
  rm /tmp/build.sh && \
  chmod +x /usr/local/share/docker-init.sh

ENTRYPOINT [ "/usr/local/share/docker-init.sh" ]
