FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-alpine

WORKDIR /build

RUN apk update && \
    apk add libx11-dev libxrandr-dev dbus-dev dbus-glib-dev pkgconfig dbus-cpp-dev dbus-x11 dbus-libs dbus && \
    rustup target add aarch64-unknown-linux-musl

ENTRYPOINT [ "/build/entrypoint.sh" ]