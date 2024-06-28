use rand::Rng;
use std::sync::Arc;

use crate::{document::Document, roles::Role};

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Alive,
    Dead,
}

#[derive(Clone, Copy, Debug)]
pub enum ResidentType {
    Human,
    Bot,
}

pub trait ResidentStrategy: Send + Sync {
    fn perform_action(&self);
}

pub struct KillerStrategy;
impl ResidentStrategy for KillerStrategy {
    fn perform_action(&self) {
        println!("Killer is performing action");
    }
}

pub struct PolicemanStrategy;
impl ResidentStrategy for PolicemanStrategy {
    fn perform_action(&self) {
        println!("Policeman is performing action");
    }
}

pub struct Resident {
    name: String,
    age: usize,
    account_balance: f64,
    status: Status,
    resident_type: ResidentType,
    documents: Vec<Document>,
    strategy: Arc<dyn ResidentStrategy>,
}

impl Resident {
    pub fn new(
        name: String,
        age: usize,
        account_balance: f64,
        strategy: Arc<dyn ResidentStrategy>,
    ) -> Resident {
        Resident {
            name,
            age,
            account_balance,
            status: Status::Alive,
            resident_type: ResidentType::Human,
            documents: Vec::new(),
            strategy,
        }
    }

    pub fn perform_action(&self) {
        self.strategy.perform_action();
    }

    pub fn describe(&self) -> String {
        format!(
            "{}, {} y.o., {:?}. Account balance: {}, status: {:?}{}",
            self.name,
            self.age,
            self.resident_type,
            self.account_balance,
            self.status,
            self.documents
                .iter()
                .enumerate()
                .map(|(dx, doc)| format!("\n{}. {}", dx, doc))
                .collect::<String>()
        )
    }
}

pub struct ResidentFactory;

impl ResidentFactory {
    pub fn create_resident(name: String, age: usize, account_balance: f64, role: Role) -> Resident {
        let strategy: Arc<dyn ResidentStrategy> = match role {
            Role::Killer => Arc::new(KillerStrategy),
            Role::Policeman => Arc::new(PolicemanStrategy),
            _ => unimplemented!(),
        };

        Resident::new(name, age, account_balance, strategy)
    }

    pub fn generate_random() -> Resident {
        let mut rng = rand::thread_rng();

        let names = vec!["Alice", "Bob", "Charlie", "Diana", "Eve"];
        let name = names[rng.gen_range(0..names.len())].to_string();
        let age = rng.gen_range(18..81);
        let account_balance = rng.gen_range(1000.0..10000.0);
        let role = match rng.gen_range(0..9) {
            0 => Role::Killer,
            1 => Role::Policeman,
            2 => Role::Doctor,
            3 => Role::Janitor,
            4 => Role::OldWoman,
            5 => Role::Swindler,
            6 => Role::Avenger,
            7 => Role::Judge,
            8 => Role::Professor,
            _ => unimplemented!(),
        };
        Self::create_resident(name, age, account_balance, role)
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::Killer // Default role, change as needed
    }
}
