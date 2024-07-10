use rand::seq::SliceRandom;

use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, roles::Role};

pub struct SwindlerStrategy;

impl SwindlerStrategy {
    fn swindle(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Swindler swindles the resident in apartment {}", target);
        // Implement the swindle logic
    }
}

impl ResidentStrategy for SwindlerStrategy {
    fn perform_action_human(
        &self,
        swindler_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(swindler_apartment, hotel);
        self.swindle(hotel, target);
        history.add_action(
            swindler_apartment,
            std::format!("{:?}", "action"),
            target,
            None,
        );
    }
    fn perform_action_bot(
        &self,
        swindler_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel.get_ready_apartments(Some(swindler_apartment)).choose(&mut rand::thread_rng()) {
            self.swindle(hotel, *target);
            history.add_action(
                swindler_apartment,
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
        Role::Swindler
    }
}
