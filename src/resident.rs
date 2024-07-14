use rand::Rng;
use std::sync::Arc;

use crate::{
    document::Document,
    game_history, hotel,
    roles::Role,
    strategies::{
        _strategy::ResidentStrategy, avenger_strategy::AvengerStrategy,
        doctor_strategy::DoctorStrategy, janitor_strategy::JanitorStrategy,
        judge_strategy::JudgeStrategy, killer_strategy::KillerStrategy,
        old_woman_strategy::OldWomanStrategy, police_strategy::PoliceStrategy,
        professor_strategy::ProfessorStrategy, swindler_strategy::SwindlerStrategy,
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Alive,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuperStatus {
    Asleep,         // alive, but sleeps full night
    Unconscious,    // sleeps night, and day (considered dead)
    Energized,      // can visit two apartments per move
    Visionary,      // enlightens with knowledge of one fact
    Metamorphosing, // changes role temporarily
    Disinterested,  // person is awake, but does not do the job
    Aggressive,     // person kills everyone (visited and visitors)
    None,
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
    pub current_position: usize, // apartment
    pub apartment_number: usize,
    pub status: Status,
    pub super_status: SuperStatus,
    pub resident_type: ResidentType,
    pub documents: Vec<Document>,
    pub strategy: Arc<dyn ResidentStrategy>,
}

impl Resident {
    pub fn new(
        name: String,
        age: usize,
        account_balance: f64,
        current_position: usize,
        strategy: Arc<dyn ResidentStrategy>,
        resident_type: ResidentType,
    ) -> Resident {
        let mut documents = Vec::new();
        documents.push(Document::new(strategy.confess_role(), name.clone(), age));
        Resident {
            name,
            age,
            account_balance,
            current_position,
            apartment_number: current_position,
            status: Status::Alive,
            super_status: SuperStatus::None,
            resident_type,
            documents,
            strategy,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.status == Status::Alive
            && self.super_status != SuperStatus::Unconscious
            && self.super_status != SuperStatus::Asleep
            && self.super_status != SuperStatus::Disinterested
    }

    pub fn perform_action(
        &mut self,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if self.status != Status::Alive {
            println!("Dead are not allowed to move...");
            return;
        }
        if self.super_status != SuperStatus::None {
            println!("Super status is not None...");
            // todo!();
            if self.super_status == SuperStatus::Disinterested
                || self.super_status == SuperStatus::Asleep
            {
                println!("Let's not move, shall we?..");
                return;
            }
        }
        hotel.apartments[self.current_position].read_mails();
        let is_human = self.resident_type == ResidentType::Human;
        self.strategy
            .perform_action(self.apartment_number, is_human, hotel, history);
    }
}

use std::fmt;

impl fmt::Display for Resident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{}, {} y.o., {:?}. Account balance: {}, status: {:?}, documents: {}",
                self.name,
                self.age,
                self.resident_type,
                self.account_balance,
                self.status,
                self.documents
                    .iter()
                    .enumerate()
                    .map(|(dx, doc)| format!("\n{}.\n{}", dx, doc))
                    .collect::<String>()
            )
        )
    }
}

pub struct ResidentFactory;

impl ResidentFactory {
    pub fn create_resident(
        name: String,
        age: usize,
        account_balance: f64,
        current_position: usize,
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

        Resident::new(
            name,
            age,
            account_balance,
            current_position,
            strategy,
            resident_type,
        )
    }

    pub fn generate_random(apartment: usize) -> Resident {
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
        Self::create_resident(
            name,
            age,
            account_balance,
            apartment,
            role,
            ResidentType::Bot,
        )
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::Killer // Default role, change as needed
    }
}
