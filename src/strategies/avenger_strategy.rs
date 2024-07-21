use rand::seq::SliceRandom;
use std::io::{self, Write};
use strum_macros::EnumIter;

use super::_strategy::ResidentStrategy;
use crate::game_history::GameHistory;
use crate::resident::{Resident, SuperStatus};
use crate::{hotel::Hotel, resident::Status, roles::Role};

#[derive(EnumIter, Debug, Clone)]
pub enum AvengerAction {
    Sleep,
    Kill,
}

pub struct AvengerStrategy;

impl AvengerStrategy {
    fn choose_action(
        &self,
        avenger_apartment: usize,
        target: usize,
        history: &GameHistory,
    ) -> AvengerAction {
        loop {
            println!("Choose an action from available options:");
            println!("1: Sleep");
            if history.has_visited(avenger_apartment, target) {
                println!("2: Kill");
            }

            let mut input = String::new();
            print!("Enter the number of your chosen action: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(1) => return AvengerAction::Sleep,
                Ok(2) if history.has_visited(avenger_apartment, target) => {
                    return AvengerAction::Kill
                }
                _ => println!("Invalid choice, please try again."),
            }
        }
    }

    fn perform_avenger_action(&self, action: AvengerAction, hotel: &mut Hotel, target: usize) {
        match action {
            AvengerAction::Sleep => {
                if let Some(resident) = &hotel.apartments[target].resident {
                    let mut resident = resident.lock().unwrap();
                    resident.super_status = SuperStatus::Asleep;
                }
                println!("Avenger puts the resident in apartment {} to sleep", target);
            }
            AvengerAction::Kill => {
                if let Some(resident) = &hotel.apartments[target].resident {
                    let mut resident = resident.lock().unwrap();
                    resident.status = Status::Dead;
                }
                println!("Avenger kills the resident in apartment {}", target);
            }
        }
    }
}

impl ResidentStrategy for AvengerStrategy {
    fn perform_action_human(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let avenger_apartment = performer.apartment_number;
        let target = self.choose_target(avenger_apartment, hotel);
        let action = self.choose_action(avenger_apartment, target, history);
        self.perform_avenger_action(action.clone(), hotel, target);
        history.add_action(
            avenger_apartment,
            std::format!("{:?}", action),
            target,
            None,
        );
    }

    fn perform_action_bot(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let avenger_apartment = performer.apartment_number;
        if let Some(target) = hotel
            .get_ready_apartments(Some(avenger_apartment))
            .choose(&mut rand::thread_rng())
        {
            let action = if history.has_visited(avenger_apartment, *target) {
                AvengerAction::Kill
            } else {
                AvengerAction::Sleep
            };
            self.perform_avenger_action(action.clone(), hotel, *target);
            history.add_action(
                avenger_apartment,
                std::format!("{:?}", action),
                *target,
                None,
            );
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Avenger
    }
}
