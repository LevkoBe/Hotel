use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

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
    fn perform_action_human(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.gossip(hotel, target);
    }
    fn perform_action_bot(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.gossip(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::OldWoman
    }
}
