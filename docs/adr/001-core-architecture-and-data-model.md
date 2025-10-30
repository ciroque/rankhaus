# ADR 001: Core Architecture and Data Model

## Status

Accepted

## Date

2025-10-30

## Context

Rankhaus is a Rust-based CLI tool and library for performing interactive stack ranking of arbitrary lists using multiple ranking strategies (Merge, Elo, Condorcet, Tournament, Active, Bradley-Terry). We need to establish the core architecture, data model, and user experience patterns that will guide implementation.

## Decision

### 1. Project Structure

**Decision**: Use a Cargo workspace with two crates:
- `rankhaus` - Library crate containing core data structures, strategy trait, file management, and serialization
- `rankhaus-cli` - Binary crate containing user interface, commands, and I/O presentation using `clap`

**Rationale**: Separates concerns, allows library reuse, and follows Rust best practices for CLI tools with substantial logic.

### 2. CLI Interaction Modes

**Decision**: Support both REPL and direct subcommand modes:
- Running `rankhaus` with no arguments enters REPL mode
- Running `rankhaus <command>` executes single command and exits
- REPL mode requires `init` or `load` before other commands are available

**Rationale**: REPL mode provides better UX for interactive ranking sessions, while direct commands support scripting and automation.

### 3. Synthetic Identifiers

**Decision**: Use 7-character lowercase alphanumeric synthetic IDs for both items and users:
- Items: e.g., `a7k9m2`, `xp4n8q`
- Users: e.g., `u7k2m9`, `u3n8p1` (u-prefixed to distinguish from items)
- IDs are generated once and never change
- 36^7 = ~78 billion possible combinations

**Rationale**:
- Stable references across all sessions (survives item/user edits and deletions)
- Human-readable (easier to debug than UUIDs)
- Compact in JSON
- Collision-resistant for reasonable list sizes
- Enables value mutability without breaking references

### 4. Data Model Structure

**Decision**: Use map/object structures with synthetic IDs as keys (not arrays):

```json
{
  "meta": { "name": "...", "type": "list", ... },
  "users": {
    "u7k2m9": { "username": "...", "display_name": "...", ... }
  },
  "items": {
    "a7k9m2": { "value": "...", "created": "..." }
  },
  "rankings": [ ... ]
}
```

**Rationale**:
- O(1) lookup by ID (critical for frequent operations)
- Structural guarantee of ID uniqueness
- Industry-standard pattern (Firebase, DynamoDB, etc.)
- Efficient updates and deletes

### 5. Ranking Results Storage

**Decision**: Support both orderings and ratings in ranking results:

```json
{
  "user_id": "u7k2m9",
  "strategy": "elo",
  "session": { "id": "...", "status": "completed", ... },
  "result": {
    "order": ["a7k9m2", "xp4n8q", "b2m5k1"],
    "ratings": {
      "a7k9m2": 1520.5,
      "xp4n8q": 1485.2,
      "b2m5k1": 1394.3
    }
  }
}
```

**Rationale**:
- Preserves information (Elo scores contain more data than just order)
- Supports strategy-specific display preferences
- Enables flexible aggregation in future
- Trait can accommodate different strategy outputs

### 6. Session State Persistence

**Decision**: Auto-save after every change (comparisons and structural changes):
- Comparisons saved immediately during ranking
- Item/user modifications saved immediately
- In-progress sessions store serialized strategy state

```json
{
  "session": {
    "status": "in_progress",
    "last_updated": "..."
  },
  "comparisons": [
    { "a": "id1", "b": "id2", "winner": "id1", "timestamp": "..." }
  ],
  "state": { /* strategy-specific serialized state */ }
}
```

**Rationale**:
- No lost work (crash-resistant)
- Clean resume experience
- Simpler mental model (file is always source of truth)
- Negligible I/O overhead for human-paced interactions

### 7. Strategy State Serialization

**Decision**: Store full strategy internal state (not replay comparisons):
- Each strategy implements serialization/deserialization
- State stored in `state` field of in-progress sessions
- On resume, deserialize and continue

**Rationale**:
- Faster resume (no replay needed)
- Simpler implementation (no replay logic to maintain)
- Avoids potential divergence between replay and live logic

### 8. Initialization Pattern

**Decision**: Require explicit initialization via `rankhaus init <name>`:
- Prompts for username and optional metadata
- Creates JSON file with proper structure
- Only `init`, `load`, `help`, and `version` available without loaded list

**Rationale**:
- Clear, intentional setup (like `git init`)
- Enforces proper state management
- Prevents confusing errors from missing context

### 9. User Management

**Decision**: Add `users` command with full CRUD operations:
- `users add <username>` - Create new user
- `users remove <username> [--cascade]` - Remove user (optionally delete rankings)
- `users edit <username> <new_display_name>` - Update display name
- `users list` - Show all users
- `users select <username>` - Set active user for session

**Rationale**:
- Supports multi-user ranking scenarios
- Explicit user management prevents confusion
- Cascade delete provides safety and flexibility

### 10. Item Value Mutability

**Decision**: Support editing item values via `items edit <identifier> <new_value>`:
- Identifier can be synthetic ID or current value
- Synthetic ID remains stable, only value changes
- All ranking references remain valid

**Rationale**:
- Enables typo fixes without data loss
- Supports display value refinement
- Justifies synthetic ID complexity

### 11. Comparison User Experience

**Decision**: Arrow keys + Enter for interactive comparison, with numeric fallback:
- Primary: Arrow keys to highlight, Enter to confirm
- Fallback: Numeric input (1/2) for non-interactive terminals
- Always support 'q' to quit ranking session

**Rationale**:
- Intuitive and fast for interactive use
- Modern, polished feel
- Accessible and scriptable via numeric fallback
- Easy to implement with `dialoguer` or `inquire` crates

### 12. Testing Strategy

**Decision**: Mixed testing approach:
- Unit tests written alongside library code implementation
- Integration tests after core functionality complete
- Property-based tests (via `proptest`) for ranking strategies
- Example-based tests for specific scenarios
- Target: 90%+ code coverage

**Rationale**:
- Catches bugs early without slowing initial development
- Property-based tests verify mathematical invariants (transitivity, consistency)
- Comprehensive coverage ensures reliability

## Consequences

### Positive

- Clear separation of concerns between library and CLI
- Robust data model supporting complex multi-user scenarios
- Crash-resistant with auto-save
- Flexible ranking result storage accommodates diverse strategies
- Intuitive UX for both interactive and scripted usage
- Stable identifiers prevent data corruption from edits

### Negative

- Synthetic IDs add complexity to JSON (less human-readable)
- Auto-save may cause issues with concurrent access (future consideration)
- Strategy serialization requires per-strategy implementation effort
- Map-based storage slightly less familiar than array-based

### Neutral

- Workspace structure requires slightly more setup than single crate
- REPL mode adds implementation complexity but improves UX
- Property-based testing has learning curve but improves quality

## Notes

- Future enhancements may include session locking for concurrent access
- Compressed storage format (`.rankhaus.gz`) deferred to later
- TUI mode with visualization deferred to later
- Remote/shared list support deferred to later
