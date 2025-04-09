use regex::Regex;
use crate::command::Command;

pub fn parse(input: &str) -> Command {
    let input = input.trim();
    
    // Basic commands
    if input == "fetch appointments" {
        return Command::FetchAppointments;
    } else if input == "commit encounter" {
        return Command::CommitEncounter;
    }
    
    // Attend appointment
    let attend_regex = Regex::new(r"^attend appointment (\d+)$").unwrap();
    if let Some(captures) = attend_regex.captures(input) {
        if let Some(num_match) = captures.get(1) {
            if let Ok(num) = num_match.as_str().parse::<usize>() {
                return Command::AttendAppointment(num);
            }
        }
    }
    
    // Register section
    let register_regex = Regex::new(r#"^register (complaint|physical exam|diagnostic hypothesis) "(.+)"$"#).unwrap();
    if let Some(captures) = register_regex.captures(input) {
        let section = captures.get(1).map_or("", |m| m.as_str()).to_string();
        let details = captures.get(2).map_or("", |m| m.as_str()).to_string();
        return Command::RegisterSection { section, details };
    }
    
    // Clinical actions
    let prescribe_regex = Regex::new(r#"^prescribe "(.+)"$"#).unwrap();
    if let Some(captures) = prescribe_regex.captures(input) {
        let details = captures.get(1).map_or("", |m| m.as_str()).to_string();
        return Command::Prescribe(details);
    }
    
    let request_regex = Regex::new(r#"^request "(.+)"$"#).unwrap();
    if let Some(captures) = request_regex.captures(input) {
        let details = captures.get(1).map_or("", |m| m.as_str()).to_string();
        return Command::Request(details);
    }
    
    let referral_regex = Regex::new(r#"^referral "(.+)"$"#).unwrap();
    if let Some(captures) = referral_regex.captures(input) {
        let details = captures.get(1).map_or("", |m| m.as_str()).to_string();
        return Command::Referral(details);
    }
    
    // If we got here, the command is unknown
    Command::Unknown(input.to_string())
}
