#[derive(Debug)]
pub enum Command {
    FetchAppointments,
    AttendAppointment(usize),
    RegisterSection { section: String, details: String },
    Prescribe(String),
    Request(String),
    Referral(String),
    CommitEncounter,
    Unknown(String),
}
