# godo

**Git-Oriented DO** - A CLI-first local TODO management tool.

Manage your tasks from anywhere in your terminal without leaving the command line.

## Features

- Single binary, no runtime dependencies
- Works completely offline
- Human-readable data format (Markdown)
- Human-readable config format (TOML)
- Cross-platform (Linux, macOS, Windows)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later

## Installation

### From source

```bash
# Clone the repository
git clone https://github.com/bklie/godo.git
cd godo

# Build and install
cargo install --path .
```

## Usage

### Initialize

Create config and data files in `~/.godo/`:

```bash
godo init
```

### Add a task

```bash
godo add "Write README"
```

### List tasks

```bash
# Show pending tasks only
godo list

# Show all tasks including completed ones
godo list --all
```

### Mark a task as done

```bash
godo done 1
```

### Edit a task

```bash
godo edit 1 "Write README.md"
```

### Remove a task

```bash
godo rm 1
```

### Help

```bash
godo --help
godo --version
```

## Data Storage

All data is stored locally in `~/.godo/`:

```
~/.godo/
├── config.toml    # Configuration file
└── tasks.md       # Task data (Markdown format)
```

### Task format

Tasks are stored in a human-readable Markdown format:

```markdown
# godo tasks

## Todo

- [ ] Task 1 <!-- id:1 created:2025-01-07T10:00:00+09:00 -->

## Done

- [x] Task 2 <!-- id:2 created:2025-01-06T09:00:00+09:00 done:2025-01-07T12:00:00+09:00 -->
```

## Build Dependencies

| Crate | Purpose |
|-------|---------|
| [clap](https://crates.io/crates/clap) | Command-line argument parsing |
| [serde](https://crates.io/crates/serde) | Serialization/deserialization |
| [toml](https://crates.io/crates/toml) | TOML config file parsing |
| [chrono](https://crates.io/crates/chrono) | Date and time handling |
| [dirs](https://crates.io/crates/dirs) | Platform-specific directories |
| [thiserror](https://crates.io/crates/thiserror) | Error handling |
| [regex](https://crates.io/crates/regex) | Markdown parsing |

## License

MIT
