# Kickbase

## Prerequisites

- [proto](https://moonrepo.dev/docs/proto/install)
- cargo-binstall
- cargo-watch
- bun 1.1.0+
- rust 1.80.0+

## Installation

```sh
proto setup
proto use
moon setup
moon sync projects
moon sync hooks
moon sync codeowners
moon sync config-schemas
moon run dev
```

### Nix

```sh
nix develop -c $SHELL
```

## Building

### Using moon

```sh
moon run build
```

### Using make

```sh
make
```

### Using nix

```sh
nix build
```

### Using cargo

```sh
cargo build
```

## Running

### Using cargo

```sh
cargo run --release
```

### Using nix

```sh
nix run
```

## Developing

### Using moon

```sh
moon run dev
```

### Using make

```sh
make dev
```

### Using cargo

```sh
cargo watch -c -w src -w templates -w styles -- bunx tailwindcss -i styles/tailwind.css -o assets/main.css && cargo run
```

