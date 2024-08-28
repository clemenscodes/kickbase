BINARY_NAME := kickbase

ifeq ($(OS),Windows_NT)
    CARGO := cargo
    RM := del /Q
    RMDIR := rmdir /S /Q
    EXEC := $(BINARY_NAME).exe
    SHELL := powershell.exe
else
    CARGO := cargo
    RM := rm -f
    RMDIR := rm -rf
    EXEC := $(BINARY_NAME)
    SHELL := /bin/sh
endif

all: build

build:
	$(CARGO) build

release:
	$(CARGO) build --release

run:
	$(CARGO) run

run-release:
	$(CARGO) run --release

test:
	$(CARGO) test

test-all:
	$(CARGO) test --all-features

fmt:
	$(CARGO) fmt

lint:
	$(CARGO) clippy

clean:
	$(CARGO) clean

doc:
	$(CARGO) doc --open

install:
	$(CARGO) install --path .

uninstall:
	$(CARGO) uninstall $(BINARY_NAME)

dist-clean: clean
	$(RMDIR) target/doc

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