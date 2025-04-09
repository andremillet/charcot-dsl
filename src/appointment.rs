use std::collections::HashMap;

pub struct Appointment {
    pub id: usize,
    pub patient_name: String,
    pub time: String,
    pub reason: String,
}

pub struct AppointmentManager {
    appointments: Vec<Appointment>,
}

impl AppointmentManager {
    pub fn new() -> Self {
        // Create a list of 10 appointments with varied information
        let appointments = vec![
            Appointment {
                id: 1,
                patient_name: "Jane Doe".to_string(),
                time: "09:00".to_string(),
                reason: "Annual checkup".to_string(),
            },
            Appointment {
                id: 2,
                patient_name: "John Smith".to_string(),
                time: "09:30".to_string(),
                reason: "Follow-up after surgery".to_string(),
            },
            Appointment {
                id: 3,
                patient_name: "Mary Johnson".to_string(),
                time: "10:15".to_string(),
                reason: "Chronic headache".to_string(),
            },
            Appointment {
                id: 4,
                patient_name: "Robert Brown".to_string(),
                time: "11:00".to_string(),
                reason: "Diabetes management".to_string(),
            },
            Appointment {
                id: 5,
                patient_name: "Patricia Davis".to_string(),
                time: "11:45".to_string(),
                reason: "Skin rash".to_string(),
            },
            Appointment {
                id: 6,
                patient_name: "Michael Wilson".to_string(),
                time: "13:30".to_string(),
                reason: "Hypertension follow-up".to_string(),
            },
            Appointment {
                id: 7,
                patient_name: "Elizabeth Martinez".to_string(),
                time: "14:15".to_string(),
                reason: "Pregnancy checkup".to_string(),
            },
            Appointment {
                id: 8,
                patient_name: "James Anderson".to_string(),
                time: "15:00".to_string(),
                reason: "Lower back pain".to_string(),
            },
            Appointment {
                id: 9,
                patient_name: "Jennifer Thomas".to_string(),
                time: "15:45".to_string(),
                reason: "Anxiety management".to_string(),
            },
            Appointment {
                id: 10,
                patient_name: "Charles Jackson".to_string(),
                time: "16:30".to_string(),
                reason: "Prescription renewal".to_string(),
            },
        ];
        
        Self { appointments }
    }
    
    pub fn get_appointments(&self) -> &Vec<Appointment> {
        &self.appointments
    }
    
    pub fn get_appointment(&self, id: usize) -> Option<&Appointment> {
        if id >= 1 && id <= self.appointments.len() {
            Some(&self.appointments[id - 1])
        } else {
            None
        }
    }
}

// New Encounter structure to track encounter data
pub struct Encounter {
    pub appointment_id: usize,
    pub patient_name: String,
    pub reason: String,
    pub sections: HashMap<String, String>,
    pub actions: Vec<String>,
    pub status: String,
}

impl Encounter {
    pub fn new(appointment_id: usize, patient_name: String, reason: String) -> Self {
        Self {
            appointment_id,
            patient_name,
            reason,
            sections: HashMap::new(),
            actions: Vec::new(),
            status: "in-progress".to_string(),
        }
    }
    
    pub fn add_section(&mut self, section: String, details: String) {
        self.sections.insert(section, details);
    }
    
    pub fn add_action(&mut self, action: String) {
        self.actions.push(action);
    }
    
    pub fn complete(&mut self) {
        self.status = "completed".to_string();
    }
}

// Manager for encounters
pub struct EncounterManager {
    pub current_encounter: Option<Encounter>,
    pub encounter_count: usize,
}

impl EncounterManager {
    pub fn new() -> Self {
        Self {
            current_encounter: None,
            encounter_count: 0,
        }
    }
    
    pub fn start_encounter(&mut self, appointment_id: usize, patient_name: String, reason: String) -> &Encounter {
        let encounter = Encounter::new(appointment_id, patient_name, reason);
        self.current_encounter = Some(encounter);
        self.current_encounter.as_ref().unwrap()
    }
    
    pub fn add_section(&mut self, section: String, details: String) -> Result<(), String> {
        match &mut self.current_encounter {
            Some(encounter) => {
                encounter.add_section(section, details);
                Ok(())
            },
            None => Err("No active encounter".to_string()),
        }
    }
    
    pub fn add_action(&mut self, action: String) -> Result<(), String> {
        match &mut self.current_encounter {
            Some(encounter) => {
                encounter.add_action(action);
                Ok(())
            },
            None => Err("No active encounter".to_string()),
        }
    }
    
    pub fn commit_encounter(&mut self) -> Result<String, String> {
        match &mut self.current_encounter {
            Some(encounter) => {
                encounter.complete();
                
                self.encounter_count += 1;
                let filename = format!("encounter_{}.med", self.encounter_count);
                
                // Convert to FHIR format
                let fhir_encounter = crate::fhir::create_encounter(
                    self.encounter_count.to_string(),
                    encounter.patient_name.clone(),
                    encounter.reason.clone(),
                    &encounter.sections,
                    &encounter.actions,
                );
                
                // Serialize to JSON
                match serde_json::to_string_pretty(&fhir_encounter) {
                    Ok(json) => {
                        // Write to file
                        match std::fs::write(&filename, json) {
                            Ok(_) => {
                                let result = filename.clone();
                                self.current_encounter = None;
                                Ok(result)
                            },
                            Err(e) => Err(format!("Failed to write file: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Failed to serialize encounter: {}", e)),
                }
            },
            None => Err("No active encounter to commit".to_string()),
        }
    }
}
