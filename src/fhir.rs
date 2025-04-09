use serde::{Deserialize, Serialize};
use chrono::Local;
use std::collections::HashMap;

// Basic FHIR Resource structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub resourceType: String,
    pub id: String,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub versionId: String,
    pub lastUpdated: String,
}

// FHIR Encounter Resource
#[derive(Serialize, Deserialize, Debug)]
pub struct Encounter {
    #[serde(flatten)]
    pub resource: Resource,
    pub status: String,
    pub class: Coding,
    pub subject: Reference,
    pub period: Period,
    pub reasonCode: Vec<CodeableConcept>,
    pub diagnosis: Vec<Diagnosis>,
    pub note: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coding {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    pub reference: String,
    pub display: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    pub start: String,
    pub end: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeableConcept {
    pub coding: Vec<Coding>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diagnosis {
    pub condition: Reference,
    pub rank: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub text: String,
}

// Helper function to create a FHIR Encounter from our internal data structures
pub fn create_encounter(
    id: String,
    patient_name: String,
    reason: String,
    sections: &HashMap<String, String>,
    actions: &Vec<String>,
) -> Encounter {
    let now = Local::now();
    
    // Create a diagnosis if there's a diagnostic hypothesis
    let diagnoses = if let Some(diagnosis) = sections.get("diagnostic hypothesis") {
        vec![Diagnosis {
            condition: Reference {
                reference: format!("Condition/{}", id),
                display: diagnosis.clone(),
            },
            rank: Some(1),
        }]
    } else {
        vec![]
    };
    
    // Create notes from sections
    let mut notes = Vec::new();
    for (section, content) in sections {
        notes.push(Annotation {
            text: format!("{}: {}", section, content),
        });
    }
    
    // Add actions as notes
    for action in actions {
        notes.push(Annotation {
            text: action.clone(),
        });
    }
    
    // Create reason code
    let reason_code = if let Some(complaint) = sections.get("complaint") {
        vec![CodeableConcept {
            coding: vec![Coding {
                system: "http://terminology.hl7.org/CodeSystem/reason-codes".to_string(),
                code: "chief-complaint".to_string(),
                display: "Chief complaint".to_string(),
            }],
            text: complaint.clone(),
        }]
    } else {
        vec![CodeableConcept {
            coding: vec![Coding {
                system: "http://terminology.hl7.org/CodeSystem/reason-codes".to_string(),
                code: "visit".to_string(),
                display: "Visit".to_string(),
            }],
            text: reason,
        }]
    };
    
    Encounter {
        resource: Resource {
            resourceType: "Encounter".to_string(),
            id: id.clone(),
            meta: Meta {
                versionId: "1".to_string(),
                lastUpdated: now.to_rfc3339(),
            },
        },
        status: "finished".to_string(),
        class: Coding {
            system: "http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string(),
            code: "AMB".to_string(),
            display: "ambulatory".to_string(),
        },
        subject: Reference {
            reference: format!("Patient/{}", id),
            display: patient_name,
        },
        period: Period {
            start: now.to_rfc3339(),
            end: Some(now.to_rfc3339()),
        },
        reasonCode: reason_code,
        diagnosis: diagnoses,
        note: notes,
    }
}
