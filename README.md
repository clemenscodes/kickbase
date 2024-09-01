# KICKBASE

A web application that displays KICKBASE data which is not available using the app.

Authentication is fully proxied to KICKBASE and
no data is ever stored on the server, no telemetry is submitted.

## Installation

There are several supported methods to install and run this project.
For unix systems, the recommended way is to use nix.

### Using nix

```sh
nix run github:clemenscodes/kickbase
```

On Windows, using nix in WSL works fine.
Alternatively, use either proto or docker.

First clone this repository.

```sh
git clone https://clemenscodes/kickbase.git
cd kickbase
```

### Using proto

First install [proto](https://moonrepo.dev/docs/proto/install)
using the instructions for your platform.

Then initialize the toolchains and build.

```sh
proto setup
proto use
moon setup
moon sync projects
moon run start
```

### Using docker

Install docker and start the container.

```sh
docker compose up
```

## Building

The server hosts styles at runtime which have to be built first using Tailwind.
Use your favorite JavaScript package manager to install `tailwindcss`.

### Build using moon

```sh
tailwindcss -i styles/tailwind.css -o assets/main.css
moon run build
```

### Build using nix

```sh
nix build
```

### Using cargo

```sh
tailwindcss -i styles/tailwind.css -o assets/main.css
cargo build --release
```

## Running

### Run using cargo

```sh
tailwindcss -i styles/tailwind.css -o assets/main.css
cargo run --release
```

### Run using nix

```sh
nix run
```

## Developing

### Develop using nix

```sh
nix develop -c $SHELL
moon run dev
```

### Develop using moon

```sh
moon run dev
```

### Develop using cargo

```sh
cargo watch -c -w src -w templates -w styles -- \
  bunx tailwindcss -i styles/tailwind.css -o assets/main.css && \
  cargo run
```
