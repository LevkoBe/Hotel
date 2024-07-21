use std::io::{self, Write};

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::_strategy::ResidentStrategy;
use crate::{
    game_history::GameHistory,
    hotel::Hotel,
    resident::{Resident, Status},
    roles::Role,
};

#[derive(EnumIter, Debug, Clone)]
pub enum KillerAction {
    Kill,
    Rob,
    Bribe,
    Threaten,
}

pub struct KillerStrategy;

impl KillerStrategy {
    fn choose_action(&self) -> KillerAction {
        loop {
            println!("Choose an action from available options:");
            for (i, action) in KillerAction::iter().enumerate() {
                println!("{}: {:?}", i + 1, action);
            }

            let mut input = String::new();
            print!("Enter the number of your chosen action: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(index) if index > 0 && index <= KillerAction::iter().count() => {
                    if let Some(action) = KillerAction::iter().nth(index - 1) {
                        return action;
                    }
                }
                _ => println!("Invalid choice, please try again."),
            }
        }
    }

    fn perform_killer_action(
        &self,
        action: KillerAction,
        hotel: &mut Hotel,
        target: usize,
        killer: &mut Resident,
    ) {
        match action {
            KillerAction::Kill => {
                if let Some(resident) = &hotel.apartments[target].resident {
                    let mut resident = resident.lock().unwrap();
                    resident.status = Status::Dead;
                }
                println!("Killer kills the resident in apartment {}", target);
            }
            KillerAction::Threaten => {
                println!("Killer threatens the resident in apartment {}", target);
                let mut mail = String::new();
                println!("Please, write the mail to the resident from the apartment:");
                io::stdin().read_line(&mut mail).unwrap();
                hotel.apartments[target].mails.push(mail);
                // todo: Implement a bit more useful threaten logic (to enforce to action)
            }
            KillerAction::Bribe => {
                println!("Killer bribes the resident in apartment {}", target);
                // Implement the bribe logic
            }
            KillerAction::Rob => {
                println!("Killer robs the resident in apartment {}", target);
                if let Some(target_resident) = &hotel.apartments[target].resident {
                    let mut res = target_resident.lock().unwrap();
                    let money = res.account_balance;
                    res.account_balance = 0.0;
                    killer.account_balance += money;
                }
                // Implement the rob logic
            }
        }
    }
}

impl ResidentStrategy for KillerStrategy {
    fn perform_action_human(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let killer_apartment = performer.apartment_number;
        let target = self.choose_target(killer_apartment, hotel);
        let action = self.choose_action();
        self.perform_killer_action(action.clone(), hotel, target, performer);
        history.add_action(killer_apartment, std::format!("{:?}", action), target, None);
    }

    fn perform_action_bot(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let killer_apartment = performer.apartment_number;
        if let Some(target) = hotel
            .get_ready_apartments(Some(killer_apartment))
            .choose(&mut rand::thread_rng())
        {
            let action = KillerAction::Kill; // Bots always choose to kill, change as needed
            self.perform_killer_action(action.clone(), hotel, *target, performer);
            history.add_action(
                killer_apartment,
                std::format!("{:?}", action),
                *target,
                None,
            );
        } else {
            println!("No available apartments to perform action");
            return;
        }
    }

    fn confess_role(&self) -> Role {
        Role::Killer
    }
}
