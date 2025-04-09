use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::Context;
use rustyline::Helper;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use std::borrow::Cow::{self, Borrowed, Owned};

// Define all available commands for autocompletion
const COMMANDS: &[&str] = &[
    "fetch appointments",
    "attend appointment",
    "register complaint",
    "register physical exam",
    "register diagnostic hypothesis",
    "prescribe",
    "request",
    "referral",
    "commit encounter",
    "exit",
];

// Define section types for register command
const SECTIONS: &[&str] = &[
    "complaint",
    "physical exam",
    "diagnostic hypothesis",
];

pub struct MedicalDSLHelper {}

impl MedicalDSLHelper {
    pub fn new() -> Self {
        MedicalDSLHelper {}
    }
}

impl Completer for MedicalDSLHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let line = line[..pos].to_lowercase();
        
        // Basic completion for commands
        if line.is_empty() {
            // If line is empty, suggest all commands
            let candidates: Vec<Pair> = COMMANDS
                .iter()
                .map(|cmd| Pair {
                    display: (*cmd).to_string(),
                    replacement: (*cmd).to_string(),
                })
                .collect();
            return Ok((0, candidates));
        }
        
        // Handle different completion scenarios based on line content
        if line.starts_with("register ") {
            // If typing a register command, suggest sections
            let register_cmd = "register ";
            if line.len() <= register_cmd.len() {
                // Suggest all sections if we just typed "register "
                let candidates: Vec<Pair> = SECTIONS
                    .iter()
                    .map(|section| {
                        let cmd = format!("register {}", section);
                        Pair {
                            display: cmd.clone(),
                            replacement: cmd,
                        }
                    })
                    .collect();
                return Ok((register_cmd.len(), candidates));
            } else {
                // Try to match partial section name
                let section_start = line.trim_start_matches(register_cmd);
                let candidates: Vec<Pair> = SECTIONS
                    .iter()
                    .filter(|section| section.starts_with(section_start))
                    .map(|section| {
                        let cmd = format!("register {}", section);
                        Pair {
                            display: cmd.clone(),
                            replacement: cmd,
                        }
                    })
                    .collect();
                if !candidates.is_empty() {
                    return Ok((register_cmd.len(), candidates));
                }
            }
        } else if line.starts_with("attend appointment ") {
            // We don't provide completion for appointment numbers as they're dynamic
            return Ok((0, vec![]));
        } else {
            // General command completion
            let candidates: Vec<Pair> = COMMANDS
                .iter()
                .filter(|cmd| cmd.starts_with(&line))
                .map(|cmd| Pair {
                    display: (*cmd).to_string(),
                    replacement: (*cmd).to_string(),
                })
                .collect();
            
            if !candidates.is_empty() {
                return Ok((0, candidates));
            }
        }

        // If no completions found, return empty list
        Ok((0, vec![]))
    }
}

impl Hinter for MedicalDSLHelper {
    type Hint = String;

    fn hint(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        // Simplified hint logic - you can expand this
        if line == "fetch" {
            Some(" appointments".to_owned())
        } else if line == "commit" {
            Some(" encounter".to_owned())
        } else if line == "register " {
            Some("complaint|physical exam|diagnostic hypothesis".to_owned())
        } else if line == "attend " {
            Some("appointment <number>".to_owned())
        } else {
            None
        }
    }
}

impl Highlighter for MedicalDSLHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        // No special highlighting for now
        Borrowed(line)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1;34m".to_owned() + hint + "\x1b[m")
    }
}

impl Validator for MedicalDSLHelper {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        // Accept all input for now
        Ok(ValidationResult::Valid(None))
    }
}

// Helper trait implementation
impl Helper for MedicalDSLHelper {}
