use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

pub struct JanitorStrategy;

impl JanitorStrategy {
    fn clean(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Janitor cleans the resident's apartment {}", target);
        // Implement the cleaning logic
    }
}

impl ResidentStrategy for JanitorStrategy {
    fn perform_action_human(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.clean(hotel, target);
    }
    fn perform_action_bot(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.clean(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Janitor
    }
}
