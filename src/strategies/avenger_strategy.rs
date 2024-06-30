use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

pub struct AvengerStrategy;

impl AvengerStrategy {
    fn avenge(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Avenger avenges the resident in apartment {}", target);
        // Implement the avenge logic
    }
}

impl ResidentStrategy for AvengerStrategy {
    fn perform_action_human(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.avenge(hotel, target);
    }
    fn perform_action_bot(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.avenge(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Avenger
    }
}
