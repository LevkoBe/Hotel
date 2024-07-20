use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, roles::Role};
use rand::seq::SliceRandom;

pub struct OldWomanStrategy;

impl OldWomanStrategy {
    fn gossip(&self, hotel: &mut hotel::Hotel, old_woman_apartment: usize, target: usize) {
        println!(
            "Old Woman gossips about the resident in apartment {}",
            target
        );

        // Move to the target apartment and fall asleep
        if let Some(apartment) = hotel.apartments.get_mut(old_woman_apartment) {
            if let Some(resident) = &apartment.resident {
                let mut resident = resident.lock().unwrap();
                println!("Old Woman has tea and falls asleep in apartment {}", target);
                resident.current_position = target;
            }
        }
        if let Some(apartment) = hotel.apartments.get_mut(target) {
            // Take a look at the documents of the resident
            if let Some(resident) = &apartment.resident {
                let resident = resident.lock().unwrap();
                println!(
                    "Old Woman looks at the documents of the resident in apartment {}:",
                    target
                );
                println!("{:?}", resident.documents);
            }
        }
    }
}

impl ResidentStrategy for OldWomanStrategy {
    fn perform_action_human(
        &self,
        old_woman_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(old_woman_apartment, hotel);
        self.gossip(hotel, old_woman_apartment, target);
        history.add_action(old_woman_apartment, "Gossip".to_string(), target, None);
    }

    fn perform_action_bot(
        &self,
        old_woman_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel
            .get_ready_apartments(Some(old_woman_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.gossip(hotel, old_woman_apartment, *target);
            history.add_action(old_woman_apartment, "Gossip".to_string(), *target, None);
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::OldWoman
    }
}
