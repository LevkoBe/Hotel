use rand::seq::SliceRandom;

use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, roles::Role};

pub struct JanitorStrategy;

impl JanitorStrategy {
    fn clean(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Janitor cleans the resident's apartment {}", target);
        // Implement the cleaning logic
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
        history.add_action(
            janitor_apartment,
            std::format!("{:?}", "action"),
            target,
            None,
        );
    }
    fn perform_action_bot(
        &self,
        janitor_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel.get_ready_apartments(Some(janitor_apartment)).choose(&mut rand::thread_rng()) {
            self.clean(hotel, *target);
            history.add_action(
                janitor_apartment,
                std::format!("{:?}", "action"),
                *target,
                None,
            );
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Janitor
    }
}
