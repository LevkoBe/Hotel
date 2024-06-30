use rand::Rng;

use super::_strategy::ResidentStrategy;
use crate::{hotel, resident::Resident, roles::Role};

pub struct ProfessorStrategy;

impl ProfessorStrategy {
    fn lecture(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Professor lectures the resident in apartment {}", target);
        // Implement the lecture logic
    }
}

impl ResidentStrategy for ProfessorStrategy {
    fn perform_action_human(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = self.choose_target(hotel);
        self.lecture(hotel, target);
    }
    fn perform_action_bot(&self, _: &Resident, hotel: &mut hotel::Hotel) {
        let target = rand::thread_rng().gen_range(0..hotel.apartments.len());
        self.lecture(hotel, target);
    }

    fn confess_role(&self) -> Role {
        Role::Professor
    }
}
