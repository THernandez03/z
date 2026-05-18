# z — Interactively Manage Your Zig Versions

> Inspired by [tj/n](https://github.com/tj/n). Written in Rust.

`z` is a simple, no-fuss Zig version manager. Download, cache, and switch between Zig versions with a single command.

## Features

- Install any released Zig version or `master` nightly builds
- Interactive version picker (arrow keys)
- Version caching — no re-downloading
- Symlink-based activation (no subshells, no profile magic)
- List local and remote versions
- Run a specific version without activating it

## Supported Platforms

| OS      | Architectures                         |
|---------|---------------------------------------|
| Linux   | x86_64, aarch64, arm, riscv64         |
| macOS   | x86_64, aarch64                       |
| Windows | x86_64, aarch64                       |

## Installation

### Pre-built binary (no Rust required)

```bash
curl -fsSL https://raw.githubusercontent.com/THernandez03/z/main/install.sh | sh
```

This installs `z` to `~/.local/bin/z`. You can override the destination:

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/THernandez03/z/main/install.sh | sh
```

### From source (requires Rust)

```bash
cargo install --git https://github.com/THernandez03/z
```

### Manual

Download the latest binary from [Releases](https://github.com/THernandez03/z/releases) and place it in your `PATH`.

## Setup

Add `~/.z/bin` to your `PATH`:

```bash
# bash / zsh
export Z_PREFIX="$HOME/.z"
export PATH="$HOME/.local/bin:$PATH"  # for the z binary
export PATH="$Z_PREFIX/bin:$PATH"     # for managed Zig binaries
```

Optional environment variables:

| Variable      | Default        | Description                          |
|---------------|----------------|--------------------------------------|
| `Z_PREFIX`    | `~/.z`         | Root installation prefix             |
| `Z_CACHE_DIR` | `~/.z/versions`| Where downloaded versions are stored |

## Usage

```
# Install a specific version
z 0.13.0
z install 0.13.0
z install master

# Interactive picker from cached versions
z

# List cached versions
z ls

# List remote versions
z ls-remote

# Download without activating
z download 0.12.1

# Show path to a cached zig binary
z which 0.13.0

# Run a specific version
z run 0.13.0 -- version

# Remove a cached version
z rm 0.12.0

# Remove all cached versions except the active one
z prune

# Uninstall active Zig
z uninstall

# Diagnostics
z doctor
```

## How It Works

`z` downloads prebuilt Zig tarballs from [ziglang.org/download](https://ziglang.org/download/index.json), caches them under `~/.z/versions/<version>/`, and creates a symlink at `~/.z/bin/zig` pointing to the selected version.

No subshells. No profile setup. Just a symlink.

## License

MIT
