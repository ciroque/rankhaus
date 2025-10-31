# Pre-canned Ranksets

This directory contains example ranksets to help users get started with Rankhaus.

## Available Ranksets

### Quick Examples (4-10 items)
- **seasons.rankset** (4 items) - Rank the four seasons
- **text-editors.rankset** (10 items) - Vim vs Emacs vs VS Code debate

### Medium Examples (15-20 items)
- **pizza-toppings.rankset** (15 items) - Pizza toppings from best to worst
- **ice-cream-flavors.rankset** (15 items) - Classic ice cream flavors
- **programming-languages.rankset** (20 items) - Popular programming languages

### Larger Examples (30+ items)
- **movies-classic.rankset** (30 items) - Greatest classic films
- **marvel-superheroes.rankset** (50 items) - Marvel heroes from comics and MCU
- **dc-superheroes.rankset** (50 items) - DC heroes from comics, movies, and TV

### Pop Culture
- **star-wars-movies.rankset** (9 items) - All Skywalker Saga films

### User-Created
- **colors.rankset** - Example with completed rankings
- **nfl-teams.rankset** - NFL teams with multiple user rankings

## Usage

Load any rankset with:
```bash
rankhaus load ranksets/seasons.rankset
```

Or in REPL mode:
```bash
rankhaus
rankhaus> load ranksets/pizza-toppings.rankset
rankhaus> rank
```

## Creating Your Own

Use these as templates for creating your own ranksets:
1. Copy an existing rankset
2. Update the metadata (name, description, author)
3. Replace the items with your own
4. Start ranking!
