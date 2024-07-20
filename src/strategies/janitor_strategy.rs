use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, resident::Resident, roles::Role};
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};

pub struct JanitorStrategy;

impl JanitorStrategy {
    fn clean(&self, hotel: &mut hotel::Hotel, target: usize) {
        println!("Janitor cleans the resident's apartment {}", target);

        // Collect data before taking a mutable borrow
        let residents_present: Vec<Arc<Mutex<Resident>>> = hotel
            .get_all_residents()
            .into_iter()
            .filter(|resident| {
                let resident = resident.lock().unwrap();
                resident.current_position == target
            })
            .collect();
        println!("Residents currently in apartment {}:", target);
        for resident in &residents_present {
            let resident = resident.lock().unwrap();
            println!("{}", resident);
        }

        if let Some(apartment) = hotel.apartments.get_mut(target) {
            // See the documents of the resident whose apartment_number is the target
            if let Some(resident) = &apartment.resident {
                let resident = resident.lock().unwrap();
                println!("Documents of the resident in apartment {}:", target);
                println!("{:?}", resident.documents);
            }
            // lock the apartment
            apartment.is_opened = false;
        }
    }
}

impl ResidentStrategy for JanitorStrategy {
    fn perform_action_human(
        &self,
        janitor_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(janitor_apartment, hotel);
        self.clean(hotel, target);
        history.add_action(janitor_apartment, "Clean".to_string(), target, None);
    }

    fn perform_action_bot(
        &self,
        janitor_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel
            .get_ready_apartments(Some(janitor_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.clean(hotel, *target);
            history.add_action(janitor_apartment, "Clean".to_string(), *target, None);
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Janitor
    }
}
