use crate::command::Command;
use crate::appointment::{AppointmentManager, EncounterManager};
use std::sync::Mutex;

// Create singleton managers
lazy_static::lazy_static! {
    static ref APPOINTMENT_MANAGER: AppointmentManager = AppointmentManager::new();
    static ref ENCOUNTER_MANAGER: Mutex<EncounterManager> = Mutex::new(EncounterManager::new());
}

pub fn execute(command: Command) -> String {
    match command {
        Command::FetchAppointments => {
            let appointments = APPOINTMENT_MANAGER.get_appointments();
            let mut result = String::new();
            
            for (i, appointment) in appointments.iter().enumerate() {
                result.push_str(&format!(
                    "{}. Patient: {}, Time: {}, Reason: {}\n",
                    i + 1,
                    appointment.patient_name,
                    appointment.time,
                    appointment.reason
                ));
            }
            
            result
        },
        
        Command::AttendAppointment(number) => {
            // Get patient from appointment manager
            match APPOINTMENT_MANAGER.get_appointment(number) {
                Some(appointment) => {
                    // Start a new encounter
                    let mut encounter_manager = ENCOUNTER_MANAGER.lock().unwrap();
                    let encounter = encounter_manager.start_encounter(
                        appointment.id, 
                        appointment.patient_name.clone(),
                        appointment.reason.clone()
                    );
                    
                    format!(
                        "Started encounter for {}\nStatus: {}\nEncounter sections initialized.",
                        encounter.patient_name,
                        encounter.status
                    )
                },
                None => format!("Error: Invalid appointment number {}", number),
            }
        },
        
        Command::RegisterSection { section, details } => {
            let mut encounter_manager = ENCOUNTER_MANAGER.lock().unwrap();
            match encounter_manager.add_section(section.clone(), details.clone()) {
                Ok(_) => format!("Added to {} section: \"{}\"", section, details),
                Err(e) => format!("Error: {}", e),
            }
        },
        
        Command::Prescribe(medication) => {
            let mut encounter_manager = ENCOUNTER_MANAGER.lock().unwrap();
            let action = format!("Prescription: {}", medication);
            match encounter_manager.add_action(action) {
                Ok(_) => format!("Prescribed: \"{}\"", medication),
                Err(e) => format!("Error: {}", e),
            }
        },
        
        Command::Request(exam) => {
            let mut encounter_manager = ENCOUNTER_MANAGER.lock().unwrap();
            let action = format!("Exam Request: {}", exam);
            match encounter_manager.add_action(action) {
                Ok(_) => format!("Requested: \"{}\"", exam),
                Err(e) => format!("Error: {}", e),
            }
        },
        
        Command::Referral(specialty) => {
            let mut encounter_manager = ENCOUNTER_MANAGER.lock().unwrap();
            let action = format!("Referral: {}", specialty);
            match encounter_manager.add_action(action) {
                Ok(_) => format!("Referred to: \"{}\"", specialty),
                Err(e) => format!("Error: {}", e),
            }
        },
        
        Command::CommitEncounter => {
            let mut encounter_manager = ENCOUNTER_MANAGER.lock().unwrap();
            match encounter_manager.commit_encounter() {
                Ok(filename) => format!("Encounter committed successfully and saved to {}", filename),
                Err(e) => format!("Error: {}", e),
            }
        },
        
        Command::Unknown(cmd) => {
            format!("Unknown command: \"{}\"", cmd)
        }
    }
}