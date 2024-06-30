use rand::Rng;
use std::sync::Arc;

use crate::{
    document::Document,
    hotel,
    roles::Role,
    strategies::{
        _strategy::ResidentStrategy, avenger_strategy::AvengerStrategy,
        doctor_strategy::DoctorStrategy, janitor_strategy::JanitorStrategy,
        judge_strategy::JudgeStrategy, killer_strategy::KillerStrategy,
        old_woman_strategy::OldWomanStrategy, police_strategy::PoliceStrategy,
        professor_strategy::ProfessorStrategy, swindler_strategy::SwindlerStrategy,
    },
};

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Alive,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResidentType {
    Human,
    Bot,
}

pub struct Resident {
    pub name: String,
    pub age: usize,
    pub account_balance: f64,
    pub status: Status,
    pub resident_type: ResidentType,
    pub documents: Vec<Document>,
    pub strategy: Arc<dyn ResidentStrategy>,
}

impl Resident {
    pub fn new(
        name: String,
        age: usize,
        account_balance: f64,
        strategy: Arc<dyn ResidentStrategy>,
        resident_type: ResidentType,
    ) -> Resident {
        Resident {
            name,
            age,
            account_balance,
            status: Status::Alive,
            resident_type,
            documents: Vec::new(),
            strategy,
        }
    }

    pub fn perform_action(&self, hotel: &mut hotel::Hotel) {
        self.strategy.perform_action(self, hotel);
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
    pub fn create_resident(
        name: String,
        age: usize,
        account_balance: f64,
        role: Role,
        resident_type: ResidentType,
    ) -> Resident {
        let strategy: Arc<dyn ResidentStrategy> = match role {
            Role::Killer => Arc::new(KillerStrategy),
            Role::Police => Arc::new(PoliceStrategy),
            Role::Doctor => Arc::new(DoctorStrategy),
            Role::Janitor => Arc::new(JanitorStrategy),
            Role::OldWoman => Arc::new(OldWomanStrategy),
            Role::Swindler => Arc::new(SwindlerStrategy),
            Role::Avenger => Arc::new(AvengerStrategy),
            Role::Judge => Arc::new(JudgeStrategy),
            Role::Professor => Arc::new(ProfessorStrategy),
            _ => unimplemented!(),
        };

        Resident::new(name, age, account_balance, strategy, resident_type)
    }

    pub fn generate_random() -> Resident {
        let mut rng = rand::thread_rng();

        let names = vec!["Alice", "Bob", "Charlie", "Diana", "Eve"];
        let name = names[rng.gen_range(0..names.len())].to_string();
        let age = rng.gen_range(18..81);
        let account_balance = rng.gen_range(1000.0..10000.0);
        let role = match rng.gen_range(0..9) {
            0 => Role::Killer,
            1 => Role::Police,
            2 => Role::Doctor,
            3 => Role::Janitor,
            4 => Role::OldWoman,
            5 => Role::Swindler,
            6 => Role::Avenger,
            7 => Role::Judge,
            8 => Role::Professor,
            _ => unimplemented!(),
        };
        Self::create_resident(name, age, account_balance, role, ResidentType::Bot)
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::Killer // Default role, change as needed
    }
}
