use super::_strategy::ResidentStrategy;
use crate::{game_history::GameHistory, hotel::Hotel, resident::Resident, roles::Role};
use rand::seq::SliceRandom;

pub struct JanitorStrategy;

impl JanitorStrategy {
    fn clean(&self, hotel: &mut Hotel, target: usize) {
        println!("Janitor cleans the resident's apartment {}", target);

        if let Some(apartment) = hotel.apartments.get_mut(target) {
            // See the documents of the resident whose apartment_number is the target
            if let Some(resident) = &apartment.resident {
                let resident = resident.lock().unwrap();
                println!("Documents of the resident in apartment {}:", target);
                println!("{:?}", resident.documents);
            }
            // lock the apartment
            apartment.is_opened = false;
        }
    }
}

impl ResidentStrategy for JanitorStrategy {
    fn perform_action_human(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let janitor_apartment = performer.apartment_number;
        let target = self.choose_target(janitor_apartment, hotel);
        self.clean(hotel, target);
        history.add_action(janitor_apartment, "Clean".to_string(), target, None);
    }

    fn perform_action_bot(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let janitor_apartment = performer.apartment_number;
        if let Some(target) = hotel
            .get_ready_apartments(Some(janitor_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.clean(hotel, *target);
            history.add_action(janitor_apartment, "Clean".to_string(), *target, None);
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Janitor
    }
}
