use rand::seq::SliceRandom;

use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, roles::Role};

pub struct PoliceStrategy;

impl PoliceStrategy {
    fn investigate(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Police investigates the resident in apartment {}", target);
        // Implement the investigation logic
    }
}

impl ResidentStrategy for PoliceStrategy {
    fn perform_action_human(
        &self,
        police_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(police_apartment, hotel);
        self.investigate(hotel, target);
        history.add_action(
            police_apartment,
            std::format!("{:?}", "action"),
            target,
            None,
        );
    }

    fn perform_action_bot(
        &self,
        police_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel
            .get_ready_apartments(Some(police_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.investigate(hotel, *target);
            history.add_action(
                police_apartment,
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
        Role::Police
    }
}
