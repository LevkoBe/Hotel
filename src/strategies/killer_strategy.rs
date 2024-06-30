use std::io::{self, Write};

use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::_strategy::ResidentStrategy;
use crate::{
    hotel,
    resident::{Resident, Status},
    roles::Role,
};

#[derive(EnumIter, Debug)]
pub enum KillerAction {
    Kill,
    Rob,
    Bribe,
    Threaten,
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
    fn perform_action_human(&self, _resident: &Resident, hotel: &mut hotel::Hotel) {
        let action = self.choose_action();
        let target = self.choose_target(hotel);
        self.perform_killer_action(action, hotel, target);
    }

    fn perform_action_bot(&self, _resident: &Resident, hotel: &mut hotel::Hotel) {
        let action = KillerAction::Kill; // Bots always choose to kill, change as needed
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.perform_killer_action(action, hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Killer
    }
}
