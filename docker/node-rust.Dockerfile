FROM node:14-buster

RUN  apt-get update && \
    apt-get install libxcb1-dev curl -y && \
    curl https://sh.rustup.rs -sSf | bash -s -- -y
