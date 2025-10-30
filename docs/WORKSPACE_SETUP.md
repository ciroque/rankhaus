# Rankhaus Workspace Setup

## Structure

The project is organized as a Cargo workspace with two crates:

```
rankhaus/
├── Cargo.toml              # Workspace root
├── rankhaus/               # Library crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── error.rs        # Error types
│       ├── id.rs           # Synthetic ID generation
│       ├── item.rs         # Item data structure
│       ├── user.rs         # User data structure
│       ├── list.rs         # List management
│       ├── ranking.rs      # Ranking results
│       ├── session.rs      # Session tracking
│       └── strategy/       # Ranking strategies
│           └── merge.rs    # Merge sort strategy (stub)
└── rankhaus-cli/           # Binary crate
    ├── Cargo.toml
    └── src/
        ├── main.rs         # CLI entry point
        ├── commands.rs     # Command execution (stubs)
        └── repl.rs         # REPL mode (stub)
```

## Dependencies

### Workspace-level Dependencies
- **serde/serde_json**: JSON serialization
- **uuid**: Session IDs
- **chrono**: Timestamps
- **anyhow/thiserror**: Error handling
- **clap**: CLI parsing
- **dialoguer/inquire**: Interactive prompts
- **proptest**: Property-based testing
- **rand**: ID generation

### Library Crate (`rankhaus`)
Core dependencies for data structures and strategies.

### Binary Crate (`rankhaus-cli`)
Depends on `rankhaus` library plus CLI/TUI dependencies.

## Features

Feature flags for optional ranking strategies:
- `default = ["merge"]`
- `merge` - Merge sort strategy (default)
- `elo` - Elo rating system
- `tournament` - Tournament/knockout
- `condorcet` - Condorcet method
- `active` - Active learning
- `btm` - Bradley-Terry model

## Building

```bash
# Check all crates
cargo check --workspace

# Build all crates
cargo build --workspace

# Run the CLI
cargo run --bin rankhaus

# Run with specific features
cargo run --bin rankhaus --features elo

# Run tests
cargo test --workspace
```

## Current Status

### ✅ Completed
- Workspace structure established
- Core data structures defined:
  - `Id` - Synthetic identifier generation
  - `Item` - Items to be ranked
  - `User` - Users who create rankings
  - `List` - Complete list with metadata
  - `Session` - Session tracking with comparisons
  - `Ranking` - Ranking results
  - `RankStrategy` - Strategy trait
- CLI command structure defined
- Basic merge strategy stub
- Error handling framework

### 🚧 In Progress
- Command implementations (all stubbed)
- REPL mode implementation
- Strategy implementations

### 📋 TODO
- Implement `init` command
- Implement `load` command
- Implement item management commands
- Implement user management commands
- Implement ranking logic
- Complete merge sort strategy
- Add additional strategies
- Write comprehensive tests
- Add integration tests

## Next Steps

1. Implement the `init` command to create new lists
2. Implement the `load` command to load existing lists
3. Build out the REPL mode
4. Implement item and user management
5. Complete the merge sort strategy
6. Add ranking execution logic
7. Write tests to achieve 90%+ coverage
