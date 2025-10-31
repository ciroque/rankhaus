# Rankhaus

A Rust-based command-line tool for performing interactive stack ranking of arbitrary lists using pairwise comparisons.

## Features

- **Interactive Ranking**: Merge sort-based pairwise comparison for efficient ranking
- **Session Management**: Save progress after each comparison, resume anytime with `sessions resume`
- **Graceful Suspend**: Press 'q' during ranking to save and exit
- **Multi-User Support**: Track rankings from multiple users with separate sessions
- **Pre-canned Examples**: 9 example ranksets included (movies, superheroes, pizza toppings, etc.)
- **REPL & Direct Modes**: Interactive REPL or scriptable direct commands
- **Auto-save**: Every change persisted immediately, no data loss
- **Flexible Data Model**: JSON-based storage with synthetic IDs for stable references

## Installation

### From Source

```bash
make release
# or
cargo build --release
```

The binary will be available at `target/release/rankhaus`.

### Install to System

```bash
make install
# or
cargo install --path rankhaus-cli
```

## Quick Start

### Browse Example Ranksets

```bash
rankhaus ranksets list
```

### Load and Rank an Example

```bash
rankhaus ranksets load ranksets/pizza-toppings.rankset
rankhaus rank
# Press 'q' to suspend, resume later with:
rankhaus sessions resume <session-id>
```

### Create Your Own Rankset

```bash
# Enter REPL mode
rankhaus

# Create a new rankset
rankhaus> ranksets new my-list --description "My favorite things"
rankhaus> items add
Item 1
Item 2
Item 3
^D
rankhaus> rank
```

## Usage

### Command Structure

```
rankhaus
├── ranksets
│   ├── list              # Browse available ranksets
│   ├── load <file>       # Load a rankset
│   └── new <name>        # Create new rankset
├── items
│   ├── list              # List all items
│   ├── add [item]        # Add items (interactive or direct)
│   ├── remove            # Remove items
│   └── edit <id> <val>   # Edit an item
├── users
│   ├── list              # List all users
│   ├── add <username>    # Add a user
│   ├── select <user>     # Set active user
│   ├── edit <user> <name> # Edit display name
│   └── remove <user>     # Remove a user
├── strategies
│   ├── list              # List strategies
│   └── select <name>     # Select strategy
├── rank                  # Start ranking (press 'q' to suspend)
└── sessions
    ├── list              # List all sessions
    ├── show <id>         # Show session details
    ├── resume <id>       # Resume in-progress session
    └── delete <id>       # Delete a session
```

### REPL Mode

Run `rankhaus` with no arguments to enter interactive mode:

```bash
$ rankhaus
Rankhaus REPL mode
Type 'help' for available commands, 'exit' to quit

No list loaded. Use 'ranksets list' to see examples or 'ranksets new <name>' to create one.

rankhaus> ranksets list
rankhaus> ranksets load ranksets/seasons.rankset
rankhaus> rank
```

### Direct Command Mode

Execute single commands without entering REPL:

```bash
rankhaus ranksets new my-list
rankhaus items add "Item 1"
rankhaus rank
```

## Pre-canned Ranksets

The `ranksets/` directory includes 9 example ranksets:

| Rankset | Items | Description |
|---------|-------|-------------|
| **seasons** | 4 | Four seasons of the year |
| **text-editors** | 10 | Popular text editors and IDEs |
| **pizza-toppings** | 15 | Pizza toppings (includes pineapple 🍍) |
| **ice-cream-flavors** | 15 | Classic ice cream flavors |
| **programming-languages** | 20 | Popular programming languages |
| **star-wars-movies** | 9 | All Skywalker Saga films |
| **movies-classic** | 30 | Greatest classic films |
| **marvel-superheroes** | 50 | Marvel heroes from comics and MCU |
| **dc-superheroes** | 50 | DC heroes from comics, movies, and TV |

Browse them with:
```bash
rankhaus ranksets list
```

## Data Format

Ranksets are stored as JSON files in `ranksets/`:

```json
{
  "meta": {
    "name": "colors",
    "author": "username",
    "description": "My favorite colors",
    "created": "2025-10-31T17:15:00Z"
  },
  "users": {
    "u7k2m9": {
      "username": "alice",
      "display_name": "Alice",
      "created": "2025-10-31T17:15:00Z",
      "last_active": "2025-10-31T17:15:00Z"
    }
  },
  "items": {
    "i7k9m2": {
      "value": "blue",
      "created": "2025-10-31T17:15:00Z"
    }
  },
  "rankings": [
    {
      "user_id": "u7k2m9",
      "strategy": "merge",
      "session": {
        "id": "s34t0qyk",
        "created": "2025-10-31T17:20:00Z",
        "last_updated": "2025-10-31T17:25:00Z",
        "status": "completed",
        "comparisons": []
      },
      "result": {
        "order": ["i7k9m2", "ixp4n8q", "ib2m5k1"]
      }
    }
  ]
}
```

**Note**: Comparisons are saved during ranking for resume capability, then cleared on completion to save space.

## Ranking Strategy

Currently implements **Merge Sort** with adaptive pairwise comparisons:
- Efficient: ~n log₂(n) comparisons for n items
- Deterministic and stable
- Progress saved after each comparison
- Can be suspended and resumed at any time

## Development

### Project Structure

```
rankhaus/
├── rankhaus/              # Library crate
│   └── src/
│       ├── error.rs       # Error types
│       ├── id.rs          # Synthetic ID generation
│       ├── item.rs        # Item data structure
│       ├── user.rs        # User data structure
│       ├── rankset.rs     # Rankset management
│       ├── ranking.rs     # Ranking results
│       ├── session.rs     # Session tracking
│       └── strategy/      # Ranking strategies
│           └── merge.rs   # Merge sort implementation
├── rankhaus-cli/          # Binary crate
│   ├── src/
│   │   ├── main.rs        # CLI entry point
│   │   ├── commands/      # Command implementations
│   │   │   ├── ranksets.rs
│   │   │   ├── items.rs
│   │   │   ├── users.rs
│   │   │   ├── rank.rs
│   │   │   └── sessions.rs
│   │   ├── repl.rs        # REPL mode
│   │   └── state.rs       # Application state
│   └── tests/
│       └── integration_test.rs
├── ranksets/              # Pre-canned example ranksets
├── Makefile               # Build automation
└── .github/workflows/     # CI/CD
    ├── ci.yml             # Continuous integration
    └── release.yml        # Automated releases
```

### Running Tests

```bash
make test
# or
cargo test --all
```

All 50 tests passing (28 unit + 22 CLI + 4 integration)

### Code Quality

```bash
make check    # Run fmt, clippy, and tests
make fmt      # Format code
make clippy   # Run lints
```

### Building

```bash
make build    # Debug build
make release  # Optimized build
make all      # Full check + release build
```

### CI/CD

GitHub Actions automatically:
- **On every push/PR**: Run tests, fmt, and clippy on Linux/macOS/Windows
- **On version tags** (`v*`): Build and publish releases for 5 platforms

Create a release:
```bash
git tag v0.1.0
git push origin v0.1.0
```

## Architecture Decisions

See [docs/adr/001-core-architecture-and-data-model.md](docs/adr/001-core-architecture-and-data-model.md) for detailed architecture decisions.

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please ensure:
- Tests pass: `make test`
- Code is formatted: `make fmt`
- Lints pass: `make clippy`
- All checks: `make check`
