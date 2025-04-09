# Charcot DSL

A domain-specific language (DSL) designed in Rust to streamline physician workflows. This DSL allows physicians to manage appointments and patient encounters using intuitive commands.

## Overview

The Charcot DSL simplifies physician workflows by providing a command-line interface with natural language commands. Key features include:

- Fetching and displaying appointment schedules
- Managing patient encounters with structured documentation
- Supporting clinical decisions (prescriptions, exams, referrals)
- Saving encounter data in FHIR-compatible JSON format
- Command autocompletion for improved productivity

The DSL prioritizes simplicity, extensibility, and integration with Rust's safety and performance features.

## Installation

### Prerequisites

- Rust programming language (2021 edition or later)
- Cargo package manager

### Building from Source

```bash
# Clone the repository
git clone https://github.com/andremillet/charcot-dsl.git
cd charcot-dsl

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

After starting the application, you'll see a prompt where you can enter commands:

```
=== Medical DSL ===
Type 'exit' to quit
Press Tab for autocompletion
> 
```

Use Tab for autocompletion and up/down arrows to navigate command history.

### Example Workflow

Here's a typical workflow using the Medical DSL:

```
> fetch appointments
1. Patient: Jane Doe, Time: 09:00, Reason: Annual checkup
2. Patient: John Smith, Time: 09:30, Reason: Follow-up after surgery
...

> attend appointment 1
Started encounter for Jane Doe
Status: in-progress
Encounter sections initialized.

> register complaint "chest pain"
Added to complaint section: "chest pain"

> register physical exam "heart rate 90 bpm, lungs clear"
Added to physical exam section: "heart rate 90 bpm, lungs clear"

> register diagnostic hypothesis "possible angina"
Added to diagnostic hypothesis section: "possible angina"

> prescribe "Aspirin 100mg daily"
Prescribed: "Aspirin 100mg daily"

> request "ECG and cardiac enzymes"
Requested: "ECG and cardiac enzymes"

> referral "Cardiology"
Referred to: "Cardiology"

> commit encounter
Encounter committed successfully and saved to encounter_1.med
```

## Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `fetch appointments` | Displays the list of scheduled appointments | `fetch appointments` |
| `attend appointment <number>` | Starts an encounter with the specified patient | `attend appointment 1` |
| `register <section> "<details>"` | Adds details to the encounter note | `register complaint "headache"` |
| `prescribe "<medication>"` | Logs a medication prescription | `prescribe "Amoxicillin 500mg"` |
| `request "<exam>"` | Logs a diagnostic exam request | `request "Chest X-ray"` |
| `referral "<specialty>"` | Logs a referral to another specialty | `referral "Cardiology"` |
| `commit encounter` | Finalizes the encounter and saves it | `commit encounter` |
| `exit` | Exits the application | `exit` |

### Register Command Sections

The `register` command supports the following sections:
- `complaint` - Patient's chief complaint
- `physical exam` - Findings from physical examination
- `diagnostic hypothesis` - Potential diagnoses

## FHIR Compatibility

The Charcot DSL saves encounter data in FHIR-compatible JSON format. Each encounter is serialized as a FHIR Encounter resource with appropriate sections, diagnoses, and clinical actions.

Encounter files are saved with a `.med` extension in the current directory and follow FHIR R4 specifications.

## Project Structure

```
medical-dsl/
├── src/
│   ├── main.rs             # Application entry point
│   ├── command.rs          # Command data structures
│   ├── parser.rs           # Command parser
│   ├── executor.rs         # Command execution logic
│   ├── appointment.rs      # Appointment and encounter management
│   ├── fhir.rs             # FHIR data structures and serialization
│   └── autocomplete.rs     # Command autocompletion
├── Cargo.toml              # Project dependencies
└── README.md               # Project documentation
```

## Future Development

Planned features for future releases:

- Integration with electronic health record (EHR) systems
- Support for additional FHIR resources
- Enhanced decision support tools
- API-based appointment fetching
- Template-based documentation
- Multi-user support
- Enhanced data visualization
- Mobile companion application

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
