use super::_strategy::ResidentStrategy;
use crate::{game_history::GameHistory, hotel::Hotel, resident::Resident, roles::Role};
use rand::seq::SliceRandom;

pub struct OldLadyStrategy;

impl OldLadyStrategy {
    fn pay_visit(&self, hotel: &mut Hotel, old_lady_apartment: usize, target: usize) {
        println!(
            "Old Lady pays a visit about the resident in apartment {}",
            target
        );

        if let Some(apartment) = hotel.apartments.get_mut(target) {
            // Take a look at the documents of the resident
            if let Some(resident) = &apartment.resident {
                let resident = resident.lock().unwrap();
                println!(
                    "Old Lady looks at the documents of the resident in apartment {}:",
                    target
                );
                println!("{:?}", resident.documents);
                println!("Old Lady has tea and falls asleep in apartment {}", target);
                apartment.guests.push(old_lady_apartment);
            }
        }
    }
}

impl ResidentStrategy for OldLadyStrategy {
    fn perform_action_human(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let old_lady_apartment = performer.apartment_number;
        let target = self.choose_target(old_lady_apartment, hotel);
        self.pay_visit(hotel, old_lady_apartment, target);
        history.add_action(old_lady_apartment, "pay_visit".to_string(), target, None);
    }

    fn perform_action_bot(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let old_lady_apartment = performer.apartment_number;
        if let Some(target) = hotel
            .get_ready_apartments(Some(old_lady_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.pay_visit(hotel, old_lady_apartment, *target);
            history.add_action(old_lady_apartment, "Pay visit".to_string(), *target, None);
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::OldLady
    }
}
