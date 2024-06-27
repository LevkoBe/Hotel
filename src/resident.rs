use crate::document::Document;
use std::sync::Arc;

#[derive(Clone)]
pub struct Resident {
    pub name: String,
    pub age: usize,
    pub account_balance: f64,
    pub role: Role,
    pub status: Status,
    pub resident_type: ResidentType,
    pub documents: Vec<Document>,
}

impl Resident {
    pub fn new(name: String, age: usize, account_balance: f64, role: Role) -> Arc<Self> {
        Arc::new(Self {
            name,
            age,
            account_balance,
            role,
            status: Status::Alive,
            resident_type: ResidentType::Human,
            documents: Vec::new(),
        })
    }

    pub fn perform_action(&self) {
        // Perform action based on role
    }
}

pub struct ResidentFactory;

impl ResidentFactory {
    pub fn create_resident(name: String, age: usize, account_balance: f64, role: Role) -> Arc<Resident> {
        Resident::new(name, age, account_balance, role)
    }
}

#[derive(Clone, Copy)]
pub enum Role {
    Killer,
    Policeman,
    Doctor,
    Janitor,
    OldWoman,
    Swindler,
    Avenger,
    Judge,
    Professor,
}

#[derive(Clone, Copy)]
pub enum Status {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub enum ResidentType {
    Human,
    Bot,
}
