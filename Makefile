BINARY_NAME := kickbase
MOON := moon

ifeq ($(OS),Windows_NT)
    RM := del /Q
    RMDIR := rmdir /S /Q
    EXEC := $(BINARY_NAME).exe
    SHELL := pwsh.exe
else
    RM := rm -f
    RMDIR := rm -rf
    EXEC := $(BINARY_NAME)
    SHELL := /bin/sh
endif

all: build

build:
	$(MOON) run build

release:
	$(MOON) run release

help:
	@echo "Usage: make [target]"
	@echo
	@echo "Targets:"
	@echo "  all            Build the project (default)"
	@echo "  build          Build the project"
	@echo "  release        Build the project with optimizations"
	@echo "  help           Display this help message"

.PHONY: all build release help
