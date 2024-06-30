use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

pub struct JudgeStrategy;

impl JudgeStrategy {
    fn judge(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Judge judges the resident in apartment {}", target);
        // Implement the judging logic
    }
}

impl ResidentStrategy for JudgeStrategy {
    fn perform_action_human(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.judge(hotel, target);
    }
    fn perform_action_bot(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.judge(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Judge
    }
}
