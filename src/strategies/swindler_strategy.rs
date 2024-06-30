use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

pub struct SwindlerStrategy;

impl SwindlerStrategy {
    fn swindle(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Swindler swindles the resident in apartment {}", target);
        // Implement the swindle logic
    }
}

impl ResidentStrategy for SwindlerStrategy {
    fn perform_action_human(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.swindle(hotel, target);
    }
    fn perform_action_bot(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.swindle(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Swindler
    }
}
