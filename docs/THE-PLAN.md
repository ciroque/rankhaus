# Rankhaus Project Seed

## Overview

`rankhaus` is a Rust-based command-line tool and library for performing interactive stack ranking of arbitrary lists of items. It supports multiple ranking strategies (e.g., Merge, Elo, Condorcet, Tournament, Active, Bradley–Terry), each implementing a shared trait interface. The CLI will allow users to load, edit, and rank lists, storing both global aggregates and per-user rankings in human-readable JSON files.

The project will be split into two crates:

* **Library (`rankhaus`)** — core data structures, strategy trait, file management, and serialization.
* **Binary (`rankhaus-cli`)** — user interface, commands, and I/O presentation using `clap`.

## Core Goals

* Provide a flexible, extensible ranking system based on pairwise comparisons.
* Use `clap` for CLI command parsing and subcommand structure.
* Support multiple strategies via a common trait.
* Maintain 90%+ test coverage.
* Persist last-used state (filename, selected strategy, etc.).
* Allow both interactive and scriptable usage.
* Store user rankings and metadata in JSON (optionally compressed).

## Command Structure

### `rankhaus items`

Manage item lists.

* `list` — show items in current list.
* `load <file>` — load list from file.
* `store [file]` — write current list to file.
* `add` — read items from stdin and append.
* `remove` — remove items by name from stdin.

### `rankhaus strategy`

Manage ranking strategy.

* `list` — list all available strategies.
* `select <strategy>` — choose the active one for the session.

### `rankhaus rank`

Perform ranking using the current strategy (interactive or scripted).

### `rankhaus sessions`

Manage ranking sessions.

* `list` — show all user sessions.
* `show <session_id>` — view session details.
* `delete <session_id>` — delete a stored session.

### `rankhaus exit`

Gracefully exit the interactive CLI; prompt to save if unsaved work exists.

---

## Data Model

### Example JSON Format

Original:

```json
{
  "meta": {
    "name": "colors",
    "type": "list",
    "author": "Ciroque",
    "description": "List of colors",
    "created": "2025-10-30T11:02:00Z"
  },
  "items": [
    { "id": 0, "value": "blue" },
    { "id": 1, "value": "red" },
    { "id": 2, "value": "green" }
  ],
  "rankings": [
    {
      "user": "Ciroque",
      "strategy": "merge",
      "session": {
        "id": "ed3e32a4-84b8-4b91-bdb4-6d1d4726df60",
        "created": "2025-10-30T11:25:00Z"
      },
      "order": [0, 2, 1]
    }
  ]
}
```

### Notes

* Each user can have multiple ranking sessions, each with a unique UUID and timestamp.
* Per-user strategies are supported.
* Global item scores may represent aggregated or consensus results.
* JSON is the default format; compressed archives (e.g., `.rankhaus.gz`) supported later.

---

## Trait Definition (Conceptual)

A shared trait defines ranking behavior for all strategies:

```rust
trait RankStrategy {
    fn name(&self) -> &'static str;
    fn compare(&mut self, a: &Item, b: &Item) -> std::cmp::Ordering;
    fn rank(&mut self, items: &mut [Item]) -> RankResult;
}

struct RankResult {
    pub order: Option<Vec<usize>>,
    pub ratings: Option<Vec<RatingEntry>>,
}

struct RatingEntry {
    pub index: usize,
    pub weight: f64,
}
```

---

## Strategy Implementations

### Core Strategies

1. **MergeSort / Pairwise Merge** (`merge`) – default strategy; efficient and intuitive for humans.
2. **Elo-Style Ranking** (`elo`) – incremental, supports continuous refinement.

### Optional Strategies (Feature-Gated)

3. **Tournament / Knockout** (`tournament`) – quick winner-finding, fun for casual ranking.
4. **Condorcet / Round-Robin** (`condorcet`) – full O(n²) comparison set, ideal for small lists.
5. **Active / Adaptive Ranking** (`active`) – dynamically selects comparisons for maximum information gain.
6. **Bradley–Terry / Thurstone–Mosteller** (`btm`) – statistical model estimating latent preference strengths.

### Feature Flags Example

```toml
[features]
default = ["merge"]
merge = []
elo = []
tournament = []
condorcet = []
active = []
btm = []
```

---

## Future Enhancements

* Session locking to prevent concurrent modification.
* Comparison logging for replay or analytics.
* Remote/shared list support (Git, HTTP, etc.).
* Aggregation commands (`rankhaus aggregate`).
* TUI mode with colors and progress visualization.
* Checkpoint/resume for incomplete ranking sessions.
* Optional binary storage format via Cargo features.

---

## Default Behavior

* Default strategy: `merge`.
* Automatically save the last used filename.
* Prompt to save on `exit` if unsaved data exists.

---

## Next Steps

1. Scaffold crate layout for `rankhaus` (lib) and `rankhaus-cli` (bin).
2. Define data structures (`Item`, `List`, `Session`, `Ranking`, etc.).
3. Implement the `RankStrategy` trait and core `MergeSortStrategy`.
4. Build out the CLI with `clap` command/subcommand hierarchy.
5. Add persistence (JSON read/write + state tracking).
6. Write integration tests targeting ≥90% coverage.
7. Implement additional strategies behind feature flags.
8. Iterate toward interactive and scriptable execution modes.
