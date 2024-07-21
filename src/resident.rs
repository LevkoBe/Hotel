use rand::Rng;
use std::sync::Arc;

use crate::{
    document::Document,
    game_history,
    hotel::Hotel,
    roles::Role,
    strategies::{
        _strategy::ResidentStrategy, avenger_strategy::AvengerStrategy,
        doctor_strategy::DoctorStrategy, janitor_strategy::JanitorStrategy,
        judge_strategy::JudgeStrategy, killer_strategy::KillerStrategy,
        old_lady_strategy::OldLadyStrategy, police_strategy::PoliceStrategy,
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
    // todo: implement special move logic
    Asleep,         // alive, but sleeps full night
    Unconscious,    // sleeps night, and day (considered dead)
    Energized,      // can visit two apartments per move
    Visionary,      // enlightens with knowledge of one fact
    Metamorphosing, // changes role temporarily
    Disinterested,  // person is awake, but does not do the job
    Aggressive,     // person kills everyone (visited and visitors)
    Arrested,       // person is unreachable
    Wounded,        // person will be dead, if not treated
    Drugged,        // person will be okay, even if wounded
    Overdosed,      // person will be dead
    None,           // person is okay
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResidentType {
    Human,
    Bot,
}

#[derive(Clone)]
pub struct Resident {
    pub name: String,
    pub age: usize,
    pub account_balance: f64,
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
        apartment_number: usize,
        strategy: Arc<dyn ResidentStrategy>,
        resident_type: ResidentType,
    ) -> Resident {
        let mut documents = Vec::new();
        documents.push(Document::new(strategy.confess_role(), name.clone(), age));
        Resident {
            name,
            age,
            account_balance,
            apartment_number,
            status: Status::Alive,
            super_status: SuperStatus::None,
            resident_type,
            documents,
            strategy,
        }
    }

    pub fn copy_fields(&mut self, other: Resident) {
        self.account_balance = other.account_balance;
        self.status = other.status;
        self.super_status = other.super_status;
        self.documents = other.documents;
    }

    pub fn is_ready(&self) -> bool {
        self.status == Status::Alive
            && self.super_status != SuperStatus::Unconscious
            && self.super_status != SuperStatus::Asleep
            && self.super_status != SuperStatus::Disinterested
    }

    pub fn update_state(&mut self) {
        match self.super_status {
            SuperStatus::Visionary => {}
            SuperStatus::Unconscious => {
                self.super_status = SuperStatus::Asleep;
            }
            SuperStatus::Wounded | SuperStatus::Overdosed => {
                self.status = Status::Dead;
            }
            _ => {
                self.super_status = SuperStatus::None;
            }
        }
    }

    pub fn perform_action(&mut self, hotel: &mut Hotel, history: &mut game_history::GameHistory) {
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
        hotel.apartments[self.apartment_number].read_mails();
        let mut self_clone = self.clone();
        self.strategy
            .perform_action(&mut self_clone, hotel, history);
        self.copy_fields(self_clone);
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

const NAMES: [&'static str; 50] = [
    "Alice Johnson",
    "Bob Smith",
    "Charlie Brown",
    "Diana White",
    "Eve Black",
    "Frank Green",
    "Grace Walker",
    "Hank Hall",
    "Ivy Adams",
    "Jack King",
    "Kathy Scott",
    "Larry Harris",
    "Mona Lewis",
    "Nate Lee",
    "Olivia Young",
    "Peter Wright",
    "Quinn Wood",
    "Rachel Fisher",
    "Sam Brooks",
    "Tina Bell",
    "Uma Evans",
    "Victor Moore",
    "Wendy Clark",
    "Xander Cole",
    "Yvonne Price",
    "Zane Murphy",
    "Amy Rogers",
    "Brian Hughes",
    "Cindy Edwards",
    "David Turner",
    "Ella Baker",
    "Fred Nelson",
    "Gina Cox",
    "Harry Carter",
    "Isla Mitchell",
    "Jason Parker",
    "Karen Roberts",
    "Liam Phillips",
    "Mia Campbell",
    "Nick Perez",
    "Oscar Russell",
    "Paula Stewart",
    "Quincy Diaz",
    "Rebecca Myers",
    "Steve Ortiz",
    "Tracy Nguyen",
    "Ursula Gray",
    "Vince Simmons",
    "Wanda Long",
    "Xenia Foster",
];

pub struct ResidentFactory;

impl ResidentFactory {
    pub fn create_resident(
        name: String,
        age: usize,
        account_balance: f64,
        apartment_number: usize,
        role: Role,
        resident_type: ResidentType,
    ) -> Resident {
        let strategy: Arc<dyn ResidentStrategy> = match role {
            Role::Killer => Arc::new(KillerStrategy),
            Role::Police => Arc::new(PoliceStrategy),
            Role::Doctor => Arc::new(DoctorStrategy),
            Role::Janitor => Arc::new(JanitorStrategy),
            Role::OldLady => Arc::new(OldLadyStrategy),
            Role::Swindler => Arc::new(SwindlerStrategy::new(None)),
            Role::Avenger => Arc::new(AvengerStrategy),
            Role::Judge => Arc::new(JudgeStrategy),
            Role::Professor => Arc::new(ProfessorStrategy),
        };

        Resident::new(
            name,
            age,
            account_balance,
            apartment_number,
            strategy,
            resident_type,
        )
    }

    pub fn generate_random(apartment: usize, role: Role) -> Resident {
        let mut rng = rand::thread_rng();

        let name = NAMES[rng.gen_range(0..NAMES.len())].to_string();
        let age = rng.gen_range(18..81);
        let account_balance = rng.gen_range(1000.0..10000.0);
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
