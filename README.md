# Begone CLI

A command-line tool to clean up project directories by removing build artifacts and dependencies for various programming languages.

## Features

- Clean Rust projects (`target/` directories)
- Clean Python projects (`.venv/`, `__pycache__/` directories)
- Clean JavaScript/TypeScript projects (`node_modules/` directories)
- Clean Java projects (`target/`, `build/` directories)
- Clean Go projects (`bin/`, `pkg/` directories)
- Clean .NET projects (`bin/`, `obj/` directories)
- Dry-run mode to preview changes
- Recursive directory scanning
- Colored output for better visibility

## Installation

### From crates.io (recommended)

```bash
cargo install begone
```

### From GitHub Releases

1. Download the appropriate binary for your system from the [latest release](https://github.com/rony0000013/begone-cli/releases/latest)
2. Make the binary executable (on Unix-like systems): `chmod +x begone-cli-{target}`
3. Move it to a directory in your `PATH`

### From Source

1. Make sure you have Rust installed (install from [rustup.rs](https://rustup.rs/))
2. Clone this repository
3. Run `cargo install --path .`

```bash
cargo install begone
```

## Usage

```
begone 0.1.0
A CLI tool to clean up project directories by removing build artifacts and dependencies

Usage: begone <COMMAND>

Commands:
  rust      Clean Rust project directories (target/)
  python    Clean Python project directories (.venv/)
  js        Clean JavaScript/TypeScript project directories (node_modules/)
  java      Clean Java project directories (target/)
  go        Clean Go project directories (bin/, pkg/)
  dotnet    Clean .NET project directories (bin/, obj/)
  all       Clean all supported project directories
  help      Print this message or the help of the given subcommand(s)

Options:
  -d, --dry-run  Run in dry-run mode (don't delete anything)
  -v, --verbose  Enable verbose output
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

Clean Rust projects recursively in the current directory:
```bash
begone rust
```

Clean Python projects in verbose mode:
```bash
begone -v python
```

Preview what would be deleted (dry run) for JavaScript/TypeScript projects:
```bash
begone --dry-run js
```

Clean all supported project types:
```bash
begone all
```

## License

MIT
