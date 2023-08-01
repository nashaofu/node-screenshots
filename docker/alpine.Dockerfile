ARG VERSION="18"

FROM node:${VERSION}-alpine

# Setup environment variables
ENV DISPLAY=":0" \
    DISPLAY_WIDTH="1280" \
    DISPLAY_HEIGHT="720"

COPY alpine-init.sh /usr/local/share/alpine-init.sh

RUN apk update && \
    apk add xvfb libxcb libxrandr

ENTRYPOINT [ "/usr/local/share/alpine-init.sh" ]

EXPOSE 6080
