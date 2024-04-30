ARG VERSION="18"

FROM node:${VERSION}-alpine

# Setup environment variables
ENV DISPLAY=":0" \
  DISPLAY_WIDTH="1280" \
  DISPLAY_HEIGHT="720"

RUN apk update && \
  apk add xvfb xfce4 libxcb libxrandr

COPY alpine-init.sh /usr/local/share/alpine-init.sh

ENTRYPOINT [ "/usr/local/share/alpine-init.sh" ]

EXPOSE 6080
