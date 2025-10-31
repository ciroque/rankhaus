# GitHub Actions Workflows

This directory contains CI/CD workflows for Rankhaus.

## Workflows

### CI (`ci.yml`)

Runs on every push to `main`/`develop` and on pull requests.

**Jobs:**
- **Format Check** - Ensures code is formatted with `cargo fmt`
- **Clippy Lints** - Runs clippy with `-D warnings` (treats warnings as errors)
- **Test Suite** - Runs all tests on Linux, macOS, and Windows
- **Build** - Builds release binaries on all platforms

### Release (`release.yml`)

Triggered when a version tag is pushed (e.g., `v0.1.0`).

**Platforms:**
- Linux x86_64
- Linux aarch64 (ARM64)
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)
- Windows x86_64

**Artifacts:**
- Compressed binaries (`.tar.gz` for Unix, `.zip` for Windows)
- SHA256 checksums for verification

## Creating a Release

1. Update version in `Cargo.toml` files
2. Commit changes
3. Create and push a tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. GitHub Actions will automatically:
   - Build binaries for all platforms
   - Create a GitHub release
   - Upload all artifacts with checksums

## Local Testing

Before pushing, run locally:
```bash
make check    # Run fmt, clippy, and tests
make all      # Run checks and build release binary
```
