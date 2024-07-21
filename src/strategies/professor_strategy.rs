use rand::seq::SliceRandom;

use super::_strategy::ResidentStrategy;
use crate::{game_history::GameHistory, hotel::Hotel, resident::Resident, roles::Role};

pub struct ProfessorStrategy;

impl ProfessorStrategy {
    fn lecture(&self, _: &mut Hotel, target: usize) {
        println!("Professor lectures the resident in apartment {}", target);
        // Implement the lecture logic
    }
}

impl ResidentStrategy for ProfessorStrategy {
    fn perform_action_human(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let professor_apartment = performer.apartment_number;
        let target = self.choose_target(professor_apartment, hotel);
        self.lecture(hotel, target);
        history.add_action(
            professor_apartment,
            std::format!("{:?}", "action"),
            target,
            None,
        );
    }
    fn perform_action_bot(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let professor_apartment = performer.apartment_number;
        if let Some(target) = hotel
            .get_ready_apartments(Some(professor_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.lecture(hotel, *target);
            history.add_action(
                professor_apartment,
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
        Role::Professor
    }
}
