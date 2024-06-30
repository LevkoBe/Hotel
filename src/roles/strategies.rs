use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

use super::{killer_actions::KillerAction, roles::Role};
use crate::{hotel, resident::Status};
use std::io::{self, Write};

pub trait ResidentStrategy: Send + Sync {
    fn perform_action(&self, hotel: &mut hotel::Hotel);
    fn confess_role(&self) -> Role;
    fn choose_target(&self, hotel: &mut hotel::Hotel) -> usize {
        println!(
            "Available apartments are: {}",
            hotel
                .available_rooms()
                .iter()
                .map(|apt| apt.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        self.get_user_input()
    }

    fn get_user_input(&self) -> usize {
        loop {
            let mut input = String::new();
            print!("Choose an apartment number: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).ok();
            match input.trim().parse::<usize>() {
                Ok(number) => return number,
                Err(_) => println!("Invalid input. Please enter a valid apartment number."),
            }
        }
    }
}

pub struct KillerStrategy;

impl KillerStrategy {
    fn choose_action(&self) -> KillerAction {
        loop {
            println!("Choose an action from available options:");
            for (i, action) in KillerAction::iter().enumerate() {
                println!("{}: {:?}", i + 1, action);
            }

            let mut input = String::new();
            print!("Enter the number of your chosen action: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(index) if index > 0 && index <= KillerAction::iter().count() => {
                    if let Some(action) = KillerAction::iter().nth(index - 1) {
                        return action;
                    }
                }
                _ => println!("Invalid choice, please try again."),
            }
        }
    }

    fn perform_killer_action(&self, action: KillerAction, hotel: &mut hotel::Hotel, target: usize) {
        match action {
            KillerAction::Kill => {
                if let Some(resident) = &hotel.apartments[target].resident {
                    let mut resident = resident.lock().unwrap();
                    resident.status = Status::Dead;
                }
                println!("Killer kills the resident in apartment {}", target);
                // Implement the kill logic
            }
            KillerAction::Threaten => {
                println!("Killer threatens the resident in apartment {}", target);
                // Implement the threaten logic
            }
            KillerAction::Bribe => {
                println!("Killer bribes the resident in apartment {}", target);
                // Implement the bribe logic
            }
            KillerAction::Rob => {
                println!("Killer robs the resident in apartment {}", target);
                // Implement the rob logic
            }
        }
    }
}

impl ResidentStrategy for KillerStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let action = self.choose_action();
        let target = self.choose_target(hotel);
        self.perform_killer_action(action, hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Killer
    }
}

pub struct PolicemanStrategy;

impl PolicemanStrategy {
    fn investigate(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!(
            "Policeman investigates the resident in apartment {}",
            target
        );
        // Implement the investigation logic
    }
}

impl ResidentStrategy for PolicemanStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.investigate(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Policeman
    }
}

pub struct DoctorStrategy;

impl DoctorStrategy {
    fn heal(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!("Doctor heals the resident in apartment {}", target);
        // Implement the healing logic
    }
}

impl ResidentStrategy for DoctorStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.heal(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Doctor
    }
}

pub struct JanitorStrategy;

impl JanitorStrategy {
    fn clean(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!("Janitor cleans the resident's apartment {}", target);
        // Implement the cleaning logic
    }
}

impl ResidentStrategy for JanitorStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.clean(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Janitor
    }
}

pub struct OldWomanStrategy;

impl OldWomanStrategy {
    fn gossip(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!(
            "Old Woman gossips about the resident in apartment {}",
            target
        );
        // Implement the gossip logic
    }
}

impl ResidentStrategy for OldWomanStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.gossip(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::OldWoman
    }
}

pub struct SwindlerStrategy;

impl SwindlerStrategy {
    fn swindle(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!("Swindler swindles the resident in apartment {}", target);
        // Implement the swindle logic
    }
}

impl ResidentStrategy for SwindlerStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.swindle(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Swindler
    }
}

pub struct AvengerStrategy;

impl AvengerStrategy {
    fn avenge(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!("Avenger avenges the resident in apartment {}", target);
        // Implement the avenge logic
    }
}

impl ResidentStrategy for AvengerStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.avenge(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Avenger
    }
}

pub struct JudgeStrategy;

impl JudgeStrategy {
    fn judge(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!("Judge judges the resident in apartment {}", target);
        // Implement the judging logic
    }
}

impl ResidentStrategy for JudgeStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.judge(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Judge
    }
}

pub struct ProfessorStrategy;

impl ProfessorStrategy {
    fn lecture(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!("Professor lectures the resident in apartment {}", target);
        // Implement the lecture logic
    }
}

impl ResidentStrategy for ProfessorStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.lecture(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Professor
    }
}
