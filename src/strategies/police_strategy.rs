use super::_strategy::ResidentStrategy;
use crate::{game_history, hotel, mail::Suspicion, roles::Role};
use rand::seq::SliceRandom;

pub struct PoliceStrategy;

impl PoliceStrategy {
    fn investigate(&self, hotel: &mut hotel::Hotel, police_apartment: usize, target: usize) {
        println!("Police investigates the resident in apartment {}", target);

        if let Some(apartment) = hotel.apartments.get_mut(target) {
            if let Some(resident) = &apartment.resident {
                let resident = resident.lock().unwrap();
                println!(
                    "Police looks at the documents of the resident in apartment {}:",
                    target
                );
                println!("{:?}", resident.documents);

                let mut is_suspicious = if resident.documents.is_empty() {
                    false
                } else if resident.documents.len() == 1 {
                    let document_role = resident.documents[0].role;
                    match document_role {
                        Role::Killer | Role::Swindler => true,
                        _ => false,
                    }
                } else {
                    true // more than one document
                };

                if !is_suspicious {
                    // Check the police suspicions
                    let mut suspicion_entries = hotel
                        .police_suspicions
                        .iter()
                        .filter(|suspicion| suspicion.suspected == target);

                    let credible_suspicion = suspicion_entries
                        .any(|suspicion| hotel.credible_sources.contains(&suspicion.from));

                    is_suspicious = suspicion_entries.count() > 1 || credible_suspicion;
                }

                if is_suspicious {
                    println!("Police suspects the resident in apartment {}", target);
                    let suspicion = Suspicion::new(
                        police_apartment,
                        target,
                        "Suspicious activity detected".to_string(),
                    );
                    hotel.investigation_queue.insert(target, suspicion);
                }
            }
        }
    }
}

impl ResidentStrategy for PoliceStrategy {
    fn perform_action_human(
        &self,
        police_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(police_apartment, hotel);
        self.investigate(hotel, police_apartment, target);
        history.add_action(police_apartment, "Investigate".to_string(), target, None);
    }

    fn perform_action_bot(
        &self,
        police_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel
            .get_ready_apartments(Some(police_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.investigate(hotel, police_apartment, *target);
            history.add_action(police_apartment, "Investigate".to_string(), *target, None);
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Police
    }
}
