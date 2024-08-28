# Makefile for a Rust project using Cargo with cross-platform support

# Define the binary name
# Update this to match the binary name specified in your Cargo.toml
BINARY_NAME := kickbase-api

# Detect the operating system
ifeq ($(OS),Windows_NT)
    # Windows-specific commands
    CARGO := cargo
    RM := del /Q
    RMDIR := rmdir /S /Q
    EXEC := $(BINARY_NAME).exe
    SHELL := powershell.exe
else
    # Unix-like commands
    CARGO := cargo
    RM := rm -f
    RMDIR := rm -rf
    EXEC := $(BINARY_NAME)
    SHELL := /bin/sh
endif

# Default target
all: build

# Build the project
build:
	$(CARGO) build

# Build the project with optimizations for release
release:
	$(CARGO) build --release

# Run the project
run:
	$(CARGO) run

# Run the project in release mode
run-release:
	$(CARGO) run --release

# Run tests for the project
test:
	$(CARGO) test

# Run tests with all features enabled
test-all:
	$(CARGO) test --all-features

# Check the code for formatting issues
fmt:
	$(CARGO) fmt

# Lint the code with Clippy
lint:
	$(CARGO) clippy

# Clean the project
clean:
	$(CARGO) clean

# Generate documentation
doc:
	$(CARGO) doc --open

# Install the binary
install:
	$(CARGO) install --path .

# Uninstall the binary
uninstall:
	$(CARGO) uninstall $(BINARY_NAME)

# Remove all build artifacts, including documentation and generated files
dist-clean: clean
	$(RMDIR) target/doc

# Display help information
help:
	@echo "Usage: make [target]"
	@echo
	@echo "Targets:"
	@echo "  all            Build the project (default)"
	@echo "  build          Build the project"
	@echo "  release        Build the project with optimizations"
	@echo "  run            Run the project"
	@echo "  run-release    Run the project in release mode"
	@echo "  test           Run tests"
	@echo "  test-all       Run tests with all features enabled"
	@echo "  fmt            Check the code for formatting issues"
	@echo "  lint           Lint the code with Clippy"
	@echo "  clean          Clean the project"
	@echo "  doc            Generate documentation"
	@echo "  install        Install the binary"
	@echo "  uninstall      Uninstall the binary"
	@echo "  dist-clean     Remove all build artifacts"
	@echo "  help           Display this help message"

.PHONY: all build release run run-release test test-all fmt lint clean doc install uninstall dist-clean help

