use rand::seq::SliceRandom;

use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, roles::Role};

pub struct DoctorStrategy;

impl DoctorStrategy {
    fn heal(&self, _: &mut hotel::Hotel, target: usize) {
        println!("Doctor heals the resident in apartment {}", target);
        // Implement the healing logic
    }
}

impl ResidentStrategy for DoctorStrategy {
    fn perform_action_human(
        &self,
        doctor_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(doctor_apartment, hotel);
        self.heal(hotel, target);
        history.add_action(
            doctor_apartment,
            std::format!("{:?}", "action"),
            target,
            None,
        );
    }
    fn perform_action_bot(
        &self,
        doctor_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel.get_ready_apartments(Some(doctor_apartment)).choose(&mut rand::thread_rng()) {
            self.heal(hotel, *target);
            history.add_action(
                doctor_apartment,
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
        Role::Doctor
    }
}
