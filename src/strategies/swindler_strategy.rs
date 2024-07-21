use super::_strategy::ResidentStrategy;
use crate::{
    document::Document,
    game_history, hotel,
    mail::Suspicion,
    roles::Role,
};
use rand::{seq::SliceRandom, Rng};

#[derive(Debug, Clone)]
pub enum SwindleSubstrategy {
    InnocentLook,
    BadGuy,
    GoodGuy,
    Collector,
    Random,
}

#[derive(Debug, Clone)]
pub struct SwindlerStrategy {
    sub_strategy: SwindleSubstrategy,
}

impl SwindlerStrategy {
    pub fn new(sub_strategy: Option<SwindleSubstrategy>) -> Self {
        let sub_strategy = sub_strategy.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            let strategies = vec![
                SwindleSubstrategy::InnocentLook,
                SwindleSubstrategy::BadGuy,
                SwindleSubstrategy::GoodGuy,
                SwindleSubstrategy::Collector,
                SwindleSubstrategy::Random,
            ];
            strategies.choose(&mut rng).unwrap().clone()
        });
        SwindlerStrategy { sub_strategy }
    }

    fn order_of_innocence() -> Vec<Role> {
        vec![
            Role::Doctor,
            Role::OldLady,
            Role::Police,
            Role::Judge,
            Role::Professor,
            Role::Janitor,
            Role::Avenger,
            Role::Swindler,
            Role::Killer,
        ]
    }

    fn good_roles() -> Vec<Role> {
        vec![
            Role::Police,
            Role::Judge,
            Role::OldLady,
            Role::Professor,
            Role::Doctor,
            Role::Janitor,
        ]
    }

    fn take_resident_documents_and_money(
        &self,
        hotel: &mut hotel::Hotel,
        apartment: usize,
        combined_documents: &mut Vec<Document>,
        combined_money: &mut f64,
    ) {
        if let Some(target_apartment) = hotel.apartments.get_mut(apartment) {
            if let Some(target_resident) = &target_apartment.resident {
                let mut target_resident = target_resident.lock().unwrap();
                *combined_money += target_resident.account_balance;
                combined_documents.extend(target_resident.documents.clone());
                target_resident.account_balance = 0.0;
                target_resident.documents.clear();
            } else {
                println!("No resident found in target apartment {}", apartment);
                return;
            }
        } else {
            println!("Target apartment {} not found", apartment);
            return;
        }
    }

    fn update_resident(
        &self,
        hotel: &mut hotel::Hotel,
        apartment: usize,
        documents: Vec<Document>,
        money: f64,
    ) {
        if let Some(swindler_apartment) = hotel.apartments.get_mut(apartment) {
            if let Some(swindler_resident) = &swindler_apartment.resident {
                let mut swindler_resident = swindler_resident.lock().unwrap();
                swindler_resident.documents = documents;
                swindler_resident.account_balance = money;
            }
        }
    }

    fn swindle_human(&self, hotel: &mut hotel::Hotel, target: usize, swindler_apartment: usize) {
        fn ask_user_which_documents_to_take(combined_documents: &Vec<Document>) -> Vec<Document> {
            let mut take_documents: Vec<_> = vec![];
            for doc in combined_documents {
                println!("Do you want to take this document? (y/n): {:?}", doc);
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                if input.trim().to_lowercase() == "y" {
                    take_documents.push(doc.clone());
                }
            }
            take_documents
        }
        fn ask_user_how_much_money_to_take(combined_money: &f64) -> f64 {
            println!(
                "How much money do you want to take? (Enter a number between 0 and {}):",
                combined_money
            );
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let take_money: f64 = input.trim().parse().unwrap_or(0.0);
            let take_money = take_money.min(*combined_money);
            take_money
        }

        println!("Swindler swindles the resident in apartment {}", target);
        let mut combined_documents: Vec<Document> = vec![];
        let mut combined_money = 0.0;

        for apartment in [target, swindler_apartment] {
            self.take_resident_documents_and_money(
                hotel,
                apartment,
                &mut combined_documents,
                &mut combined_money,
            );
        }
        let take_documents = ask_user_which_documents_to_take(&combined_documents);
        let take_money = ask_user_how_much_money_to_take(&combined_money);

        let resident_documents = combined_documents
            .iter()
            .filter(|d| !take_documents.contains(d))
            .cloned()
            .collect();
        let resident_money = combined_money - take_money;

        self.update_resident(hotel, target, resident_documents, resident_money);
        self.update_resident(hotel, swindler_apartment, take_documents, take_money);
    }

    fn swindle_bot(&self, hotel: &mut hotel::Hotel, target: usize, swindler_apartment: usize) {
        println!("Swindler swindles the resident in apartment {}", target);
        let mut combined_documents: Vec<Document> = vec![];
        let mut combined_money = 0.0;
        let mut take_documents: Vec<Document> = vec![];
        let mut take_money = 0.0;

        let good_resident = if let Some(resident) = hotel
            .apartments
            .get_mut(target)
            .and_then(|a| a.resident.as_ref())
        {
            let resident = resident.lock().unwrap();
            SwindlerStrategy::good_roles().contains(&resident.strategy.confess_role())
        } else {
            false
        };

        // Collect documents and money from both the target and swindler's apartments
        for apartment in [target, swindler_apartment] {
            self.take_resident_documents_and_money(
                hotel,
                apartment,
                &mut combined_documents,
                &mut combined_money,
            );
        }

        if !combined_documents.is_empty() {
            // Sort documents by order of innocence (0th = most innocent)
            combined_documents.sort_by_key(|d| {
                SwindlerStrategy::order_of_innocence()
                    .iter()
                    .position(|&r| r == d.role)
                    .unwrap_or(usize::MAX)
            });

            // Determine which documents to take based on the sub-strategy
            take_documents = match &self.sub_strategy {
                SwindleSubstrategy::InnocentLook => {
                    // Switch to more innocent role if possible
                    vec![combined_documents.remove(0)]
                }
                SwindleSubstrategy::BadGuy => {
                    // Leave bad document for each good character and vice versa
                    if good_resident {
                        // Take all the documents but last
                        combined_documents
                            .drain(..combined_documents.len() - 1)
                            .collect()
                    } else {
                        // Take all the documents but first
                        combined_documents.drain(1..).collect()
                    }
                }
                SwindleSubstrategy::GoodGuy => {
                    // Leave half money to good characters and notify the police about Killers
                    take_money = combined_money / 2.0;

                    for doc in &combined_documents {
                        if doc.role == Role::Killer {
                            let suspicion = Suspicion {
                                from: swindler_apartment,
                                suspected: target,
                                description: format!(
                                    "Swindler informs about a killer in apartment {}",
                                    target
                                ),
                                for_votes: 0,
                                against_votes: 0,
                            };
                            hotel.police_suspicions.push(suspicion);
                        }
                    }
                    // Take the first document
                    vec![combined_documents.remove(0)]
                }
                SwindleSubstrategy::Collector => {
                    // Collect all documents
                    vec![]
                }
                SwindleSubstrategy::Random => {
                    // Randomly choose documents to take
                    let mut rng = rand::thread_rng();
                    let num_documents_to_take = rng.gen_range(1..=combined_documents.len());
                    let mut taken_documents = vec![];
                    for _ in 0..num_documents_to_take {
                        taken_documents.push(
                            combined_documents.remove(rng.gen_range(0..combined_documents.len())),
                        );
                    }
                    taken_documents
                }
            }
        }

        // Update the residents with the new documents and money
        self.update_resident(
            hotel,
            target,
            combined_documents,
            combined_money - take_money,
        );
        self.update_resident(hotel, swindler_apartment, take_documents, take_money);
    }
}

impl ResidentStrategy for SwindlerStrategy {
    fn perform_action_human(
        &self,
        swindler_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let target = self.choose_target(swindler_apartment, hotel);
        self.swindle_human(hotel, target, swindler_apartment);
        history.add_action(swindler_apartment, "Swindle".to_string(), target, None);
    }

    fn perform_action_bot(
        &self,
        swindler_apartment: usize,
        hotel: &mut hotel::Hotel,
        history: &mut game_history::GameHistory,
    ) {
        if let Some(target) = hotel
            .get_ready_apartments(Some(swindler_apartment))
            .choose(&mut rand::thread_rng())
        {
            self.swindle_bot(hotel, *target, swindler_apartment);
            history.add_action(swindler_apartment, "Swindle".to_string(), *target, None);
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Swindler
    }
}
