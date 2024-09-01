#### BASE STAGE
#### Installs proto.

FROM rust:1.80.1-slim-bullseye AS base

# Set environment variable to avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ENV SHELL=/bin/bash

#Update the package list and install curl and proto dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git=1:2.30.2-1* \
    gzip=1.10-4* \
    unzip=6.0-26* \
    xz-utils=5.2.5-2.1* \
    curl=7.74.0-1.3* \
    pkg-config=0.29.2-1* \
    openssl=1.1.1* \
    libssl-dev=1.1.1* \
    && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install proto binary
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- 0.40.4 --yes

ENV PATH="/root/.proto/bin:$PATH"
ENV PATH="/root/.cargo/bin:$PATH"

RUN proto plugin add moon "https://raw.githubusercontent.com/moonrepo/moon/master/proto-plugin.toml" && \
  proto install moon

#### BUILD STAGE
#### Builds the project.

FROM base AS build

# Copy toolchain
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .moon .moon
COPY dockerManifest.json dockerManifest.json

# Build only dependencies
RUN rm .moon/toolchain.yml && \
  mv .moon/docker.toolchain.yml .moon/toolchain.yml && \
  echo "id: kickbase" > moon.yml && \
  echo "project:" >> moon.yml && \
  echo "  name: kickbase" >> moon.yml && \
  echo "  description: kickbase" >> moon.yml && \
  moon docker setup && \
  mkdir src/ && \
  echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
  cargo build --release

COPY tailwind.config.js tailwind.config.js
COPY moon.yml moon.yml
COPY styles styles
COPY assets assets
COPY templates templates
COPY src src

# Build application
RUN moon run kickbase:release && \
  mv target/release/kickbase . && \
  rm -rf target/release/deps/kickbase*

#### START STAGE
#### Runs the project.

FROM alpine:3.20.2 AS start

# Copy built sources
COPY --from=build /app/kickbase /usr/local/bin/kickbase

CMD ["/usr/local/bin/kickbase"]
