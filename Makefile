.PHONY: help build release test install clean fmt clippy check all

# Default target
help:
	@echo "Rankhaus - Makefile targets:"
	@echo ""
	@echo "  make build      - Build debug binary"
	@echo "  make release    - Build optimized release binary"
	@echo "  make test       - Run all tests"
	@echo "  make install    - Install release binary to ~/.cargo/bin"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make fmt        - Format code"
	@echo "  make clippy     - Run clippy lints"
	@echo "  make check      - Run fmt, clippy, and test"
	@echo "  make all        - Run check and build release"
	@echo ""

# Build debug binary
build:
	cargo build

# Build optimized release binary
release:
	cargo build --release

# Run all tests
test:
	cargo test --all

# Install to system
install: release
	cargo install --path rankhaus-cli

# Clean build artifacts
clean:
	cargo clean
	rm -f .rankhaus_history

# Format code
fmt:
	cargo fmt --all

# Run clippy lints
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (fmt, clippy, test)
check: fmt clippy test

# Build everything
all: check release
	@echo "✓ All checks passed and release binary built"
	@echo "Binary location: target/release/rankhaus"
