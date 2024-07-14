use rand::seq::SliceRandom;

use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, roles::Role};

pub struct OldWomanStrategy;

impl OldWomanStrategy {
    fn gossip(&self, _: &mut hotel::Hotel, target: usize) {
        println!(
            "Old Woman gossips about the resident in apartment {}",
            target
        );
        // Implement the gossip logic
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
        self.gossip(hotel, target);
        history.add_action(
            old_woman_apartment,
            std::format!("{:?}", "action"),
            target,
            None,
        );
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
            self.gossip(hotel, *target);
            history.add_action(
                old_woman_apartment,
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
        Role::OldWoman
    }
}
