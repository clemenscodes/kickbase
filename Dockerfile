#### BASE STAGE
#### Installs moon.

FROM rust:1.80.1-slim-bullseye AS base

# Set environment variable to avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ENV SHELL=/bin/bash

#Update the package list and install curl and proto dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    bash=5.1-2* \
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

RUN proto plugin add moon "https://raw.githubusercontent.com/moonrepo/moon/master/proto-plugin.toml" && \
  proto install moon

#### SKELETON STAGE
#### Scaffolds repository skeleton structures.

FROM base AS skeleton

# Copy entire repository and scaffold
COPY . .
RUN moon docker scaffold kickbase

#### BUILD STAGE
#### Builds the project.

FROM base AS build

# Copy toolchain
COPY --from=skeleton /root/.proto /root/.proto

# Copy workspace configs
COPY --from=skeleton /app/.moon/docker/workspace .

# Copy project sources
COPY --from=skeleton /app/.moon/docker/sources .

# Install dependencies
RUN moon docker setup && \
  moon run kickbase:release && \
  moon docker prune

#### START STAGE
#### Runs the project.

FROM base AS start

# Copy built sources
COPY --from=build /root/.proto /root/.proto
COPY --from=build /app /app

CMD ["moon", "run",  "kickbase:prod"]

