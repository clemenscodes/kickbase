#### BASE STAGE
#### Installs moon.

FROM rust:1.80.1-slim-bullseye AS base

# Set environment variable to avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Update the package list and install curl
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl=7.74.0-* tar && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install moon binary
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN curl --version
RUN curl -fsSL https://moonrepo.dev/install/moon.sh | bash
ENV PATH="/root/.moon/bin:$PATH"

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

CMD ["moon", "run",  "kickbase:start"]

