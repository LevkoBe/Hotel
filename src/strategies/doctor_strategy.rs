use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

pub struct DoctorStrategy;

impl DoctorStrategy {
    fn heal(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Doctor heals the resident in apartment {}", target);
        // Implement the healing logic
    }
}

impl ResidentStrategy for DoctorStrategy {
    fn perform_action_human(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.heal(hotel, target);
    }
    fn perform_action_bot(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.heal(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Doctor
    }
}
