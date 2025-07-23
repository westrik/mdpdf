FROM node:20-bookworm-slim

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

ARG CARGO_NIGHTLY_VERSION=2025-07-21

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-$CARGO_NIGHTLY_VERSION \
    && . "$HOME/.cargo/env"
RUN corepack enable
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

ARG CARGO_PROFILE_RELEASE_OPT_LEVEL=1
ARG CARGO_PROFILE_RELEASE_LTO=true
ARG CARGO_PROFILE_RELEASE_STRIP=symbols
ARG CARGO_PROFILE_RELEASE_PANIC=panic
ARG CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1

COPY package.json yarn.lock /app/
RUN yarn install
COPY src Cargo.* build.rs fuzz fonts index.* /app/
RUN ls && yarn build

ENV PROFILE=true

CMD ["yarn", "test"]
