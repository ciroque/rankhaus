# Rankhaus

A Rust-based command-line tool and library for performing interactive stack ranking of arbitrary lists using multiple ranking strategies.

## Features

- **Multiple Ranking Strategies**: Merge sort, Elo, Tournament, Condorcet, Active Learning, Bradley-Terry
- **Interactive & Scriptable**: REPL mode for interactive sessions, direct commands for scripting
- **Multi-User Support**: Track rankings from multiple users with separate sessions
- **Persistent State**: Auto-save after every change, resume interrupted sessions
- **Flexible Data Model**: JSON-based storage with synthetic IDs for stable references
- **Extensible**: Trait-based strategy system for easy addition of new algorithms

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/rankhaus`.

## Quick Start

### Initialize a new list

```bash
rankhaus init colors
```

### Enter REPL mode

```bash
rankhaus
```

### Use direct commands

```bash
rankhaus items add
rankhaus rank
rankhaus sessions list
```

## Usage

### Commands

- **`init <name>`** - Initialize a new ranking list
- **`load <file>`** - Load an existing list
- **`items`** - Manage items (add, remove, edit, list)
- **`users`** - Manage users (add, remove, edit, list, select)
- **`strategy`** - Select ranking strategy (list, select)
- **`rank`** - Perform interactive ranking
- **`sessions`** - View ranking sessions (list, show, delete)

### REPL Mode

Run `rankhaus` with no arguments to enter interactive mode:

```
$ rankhaus
rankhaus> init colors
rankhaus> items add
blue
red
green
^D
rankhaus> rank
```

## Data Format

Lists are stored as JSON files with the following structure:

```json
{
  "meta": {
    "name": "colors",
    "type": "list",
    "author": "username",
    "description": "My favorite colors",
    "created": "2025-10-30T11:02:00Z"
  },
  "users": {
    "u7k2m9": {
      "username": "alice",
      "display_name": "Alice",
      "created": "2025-10-30T11:02:00Z",
      "last_active": "2025-10-30T14:35:00Z"
    }
  },
  "items": {
    "a7k9m2": {
      "value": "blue",
      "created": "2025-10-30T11:02:00Z"
    }
  },
  "rankings": [
    {
      "user_id": "u7k2m9",
      "strategy": "merge",
      "session": {
        "id": "ed3e32a4-84b8-4b91-bdb4-6d1d4726df60",
        "created": "2025-10-30T11:25:00Z",
        "status": "completed"
      },
      "result": {
        "order": ["a7k9m2", "xp4n8q", "b2m5k1"]
      }
    }
  ]
}
```

## Ranking Strategies

### Default Strategies

- **Merge** (default) - Merge sort based pairwise comparison
- **Elo** - Elo rating system with incremental updates

### Optional Strategies (Feature-Gated)

Enable with `--features`:

```bash
cargo build --features tournament
cargo build --features condorcet
cargo build --features active
cargo build --features btm
```

## Development

### Project Structure

```
rankhaus/
├── rankhaus/           # Library crate
│   └── src/
│       ├── error.rs    # Error types
│       ├── id.rs       # Synthetic ID generation
│       ├── item.rs     # Item data structure
│       ├── user.rs     # User data structure
│       ├── list.rs     # List management
│       ├── ranking.rs  # Ranking results
│       ├── session.rs  # Session tracking
│       └── strategy/   # Ranking strategies
└── rankhaus-cli/       # Binary crate
    └── src/
        ├── main.rs     # CLI entry point
        ├── commands.rs # Command execution
        └── repl.rs     # REPL mode
```

### Running Tests

```bash
cargo test --workspace
```

Target: 90%+ code coverage

### Building Documentation

```bash
cargo doc --workspace --open
```

## Architecture Decisions

See [docs/adr/001-core-architecture-and-data-model.md](docs/adr/001-core-architecture-and-data-model.md) for detailed architecture decisions.

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please ensure:
- Tests pass: `cargo test --workspace`
- Code is formatted: `cargo fmt --all`
- Lints pass: `cargo clippy --workspace`
