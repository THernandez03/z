# z — Interactively Manage Your Zig Versions

`z` is a simple, no-fuss Zig version manager. Download, cache, and switch between Zig versions with a single command.

## Features

- Install any released Zig version or `master` nightly builds
- Interactive version picker (arrow keys)
- Version caching — no re-downloading
- Symlink-based activation (no subshells, no profile magic)
- List local and remote versions
- Run a specific version without activating it

## Supported Platforms

| OS      | Architectures                 |
| ------- | ----------------------------- |
| Linux   | x86_64, aarch64, arm, riscv64 |
| macOS   | x86_64, aarch64               |
| Windows | x86_64, aarch64               |

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

| Variable      | Default         | Description                          |
| ------------- | --------------- | ------------------------------------ |
| `Z_PREFIX`    | `~/.z`          | Root installation prefix             |
| `Z_CACHE_DIR` | `~/.z/versions` | Where downloaded versions are stored |

## Usage

```bash
# Install and activate a version
z 0.13.0
z master
z latest

# Interactive picker from cached versions
z

# List cached versions
z ls

# List remote versions
z ls-remote

# Fetch into cache without activating
z fetch 0.12.1

# Show path to a cached zig binary
z which 0.13.0

# Run a specific version
z run 0.13.0 -- version

# Remove a cached version (interactive picker if no version given)
z remove 0.12.0
z rm 0.12.0         # alias

# Remove all cached versions except the active one
z prune

# Also remove the active version
z prune --force

# Show info
z info

# Update z itself
z update

# Fully remove z + all cached versions (requires confirmation)
z uninstall
z uninstall --yes   # skip confirmation prompt
```

## Version Aliases

| Alias    | Resolves to            |
| -------- | ---------------------- |
| `master` | Latest nightly build   |
| `latest` | Same as `master`       |
| `0.13`   | Latest patch in 0.13.x |

## How It Works

`z` downloads prebuilt Zig tarballs from [ziglang.org/download](https://ziglang.org/download/index.json), caches them under `~/.z/versions/<version>/`, and creates a symlink at `~/.z/bin/zig` pointing to the selected version.

No subshells. No profile setup. Just a symlink.

## Related Projects

| Project                                | Runtime                 |
| -------------------------------------- | ----------------------- |
| [n](https://github.com/THernandez03/n) | Node.js version manager |
| [b](https://github.com/THernandez03/b) | Bun version manager     |
| [d](https://github.com/THernandez03/d) | Deno version manager    |
| [r](https://github.com/THernandez03/r) | Rust version manager    |

## License

MIT
