# Charcot DSL

A domain-specific language (DSL) for development of medical applications.

## Overview

Charcot is a specialized language designed to simplify common tasks in medical software development. Named after the neurologist Jean-Martin Charcot, this DSL provides intuitive commands for healthcare professionals and developers to interact with medical data without needing to understand complex database queries or API structures.

## Core Concepts

Charcot DSL is built around:

- **Commands**: Simple, intuitive actions (like `fetch`)
- **Data Sources**: Connections to databases, APIs, and local files
- **Pattern-Based Parsing**: Converting human-readable commands to executable code

## Project Structure

```
charcot_dsl/
├── commands/           # Command definitions in Charcot DSL
│   ├── fetch.charcot
│   └── other_command.charcot
└── parser/             # Command implementations in Rust
    ├── fetch.rs
    └── other_command.rs
```

Each command is defined in its own `.charcot` file, with corresponding implementation in a `.rs` file. This separation allows:

- Domain experts to review and modify the command syntax
- Developers to implement the technical details
- Easy extension by adding new command pairs

## Commands

### `fetch`

Retrieves medical data, primarily focused on appointments.

**Syntax:**
```
fetch appointments [timeframe]
```

Where `timeframe` can be:
- `day` (default)
- `week`
- `month`

**Behavior:**
- Searches for appointment data in available sources (database, API, local files)
- Filters records where `encounter.date` matches the current date (for `day`)
- Returns a JSON list of matching appointments

**Example:**
```
fetch appointments day
```
Returns all appointments scheduled for today.

## Implementation Approach

Charcot uses pattern-based parsing where:
1. `.charcot` files define command syntax, structure, and expected behaviors
2. `.rs` files implement the parsing logic and execution
3. A unified parser converts user commands into appropriate actions

## REPL Environment

Charcot includes an interactive REPL (Read-Eval-Print Loop) for testing commands and experimenting with the DSL:

```
charcot_dsl/
├── repl/
│   ├── main.rs           # REPL entry point
│   ├── interpreter.rs    # Command execution engine
│   ├── formatter.rs      # Output formatting
│   └── helpers/          # REPL utilities
```

### Using the REPL

Start the REPL with:

```bash
cargo run --bin charcot-repl
```

The REPL provides:
- Interactive command execution
- Immediate feedback
- Command history
- Tab completion for commands and parameters
- Help documentation with `help <command>`
- Simulated data sources for testing

### Example REPL Session

```
Charcot DSL v0.1.0
Type 'help' for available commands or 'exit' to quit.

charcot> help fetch
COMMAND: fetch
  Retrieves medical data from available sources.
  
  USAGE:
    fetch <entity> [timeframe] [from <source>]
    fetch <entity> period(YYYY-MM-DD to YYYY-MM-DD) [from <source>]
  
  EXAMPLES:
    fetch appointments
    fetch appointments week
    fetch appointments period(2025-04-01 to 2025-04-15)

charcot> fetch appointments
[
  {
    "encounter_id": "E12345",
    "patient_id": "P56789",
    "encounter_date": "2025-04-11",
    "provider_id": "DR001",
    "status": "scheduled",
    "type": "follow-up",
    "notes": "Regular checkup",
    "location": "Room 302"
  },
  ...
]

charcot> 
```

## Development Status

This project is in early development. The `fetch` command pattern is the first being implemented, with more commands to follow.