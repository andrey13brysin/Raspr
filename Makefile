.PHONY: all build run test release clean help

all: build

build:
	cargo build

run:
	cargo run

test:
	cargo test

release:
	cargo build --release

clean:
	cargo clean

help:
	@echo "Targets:"
	@echo "  build    Build debug binary"
	@echo "  run      Run the program"
	@echo "  test     Run tests"
	@echo "  release  Build release binary"
	@echo "  clean    Remove build artifacts"
