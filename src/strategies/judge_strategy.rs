use rand::seq::SliceRandom;

use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, roles::Role};

pub struct JudgeStrategy;

impl JudgeStrategy {
    fn judge(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Judge judges the resident in apartment {}", target);
        // Implement the judging logic
    }
}

impl ResidentStrategy for JudgeStrategy {
    fn perform_action_human(
        &self,
        judge_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(judge_apartment, hotel);
        self.judge(hotel, target);
        history.add_action(
            judge_apartment,
            std::format!("{:?}", "action"),
            target,
            None,
        );
    }
    fn perform_action_bot(
        &self,
        judge_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel.get_ready_apartments(Some(judge_apartment)).choose(&mut rand::thread_rng()) {
            self.judge(hotel, *target);
            history.add_action(
                judge_apartment,
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
        Role::Judge
    }
}
