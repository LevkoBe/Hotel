use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

pub struct PoliceStrategy;

impl PoliceStrategy {
    fn investigate(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Police investigates the resident in apartment {}", target);
        // Implement the investigation logic
    }
}

impl ResidentStrategy for PoliceStrategy {
    fn perform_action_human(&self, _resident: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.investigate(hotel, target);
    }

    fn perform_action_bot(&self, _resident: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.investigate(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Police
    }
}
