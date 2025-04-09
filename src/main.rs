mod command;
mod parser;
mod executor;
mod appointment;
mod fhir;
mod autocomplete;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use autocomplete::MedicalDSLHelper;
use rustyline::Config;

fn main() -> Result<()> {
    println!("=== Medical DSL ===");
    println!("Type 'exit' to quit");
    println!("Press Tab for autocompletion");
    
    // Configure rustyline with our custom helper
    let config = Config::builder()
        .auto_add_history(true)
        .build();
    let mut rl = DefaultEditor::with_config(config)?;
    
    // Enable tab completion
    let helper = MedicalDSLHelper::new();
    rl.set_helper(Some(helper));

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input == "exit" {
                    break;
                }
                
                let command = parser::parse(input);
                let output = executor::execute(command);
                println!("{}", output);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    
    println!("Goodbye!");
    Ok(())
}
