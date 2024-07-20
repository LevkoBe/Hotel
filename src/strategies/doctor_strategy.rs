use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, resident::SuperStatus, roles::Role};
use rand::seq::SliceRandom;

pub struct DoctorStrategy;

impl DoctorStrategy {
    fn heal(&self, hotel: &mut hotel::Hotel, target: usize) {
        if let Some(resident) = &hotel.apartments[target].resident {
            let mut resident = resident.lock().unwrap();
            match resident.super_status {
                SuperStatus::Drugged => {
                    resident.super_status = SuperStatus::Overdosed;
                    println!(
                        "Doctor heals the resident in apartment {}. They are now Overdosed.",
                        target
                    );
                }
                _ => {
                    resident.super_status = SuperStatus::Drugged;
                    println!(
                        "Doctor heals the resident in apartment {}. They are now Drugged.",
                        target
                    );
                }
            }
        }
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
        history.add_action(doctor_apartment, "Heal".to_string(), target, None);
    }

    fn perform_action_bot(
        &self,
        doctor_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel
            .get_ready_apartments(Some(doctor_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.heal(hotel, *target);
            history.add_action(doctor_apartment, "Heal".to_string(), *target, None);
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Doctor
    }
}
