use crate::{
    game_history,
    hotel::Hotel,
    resident::{Resident, ResidentType},
    roles::Role,
};
use std::io::{self, Write};

pub trait ResidentStrategy: Send + Sync {
    fn perform_action(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut game_history::GameHistory,
    ) {
        let is_human = performer.resident_type == ResidentType::Human;
        if is_human {
            self.perform_action_human(performer, hotel, history);
        } else {
            self.perform_action_bot(performer, hotel, history);
        }
    }

    fn perform_action_human(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut game_history::GameHistory,
    );
    fn perform_action_bot(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut game_history::GameHistory,
    );

    fn confess_role(&self) -> Role;

    fn choose_target(&self, own_apartment: usize, hotel: &mut Hotel) -> usize {
        let available_apartments = hotel.available_rooms();
        println!(
            "Available apartments are: {}",
            available_apartments
                .iter()
                .map(|apt| apt.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        self.get_user_input(available_apartments, own_apartment)
    }

    fn get_user_input(&self, available_apartments: Vec<usize>, own_apartment: usize) -> usize {
        loop {
            let mut input = String::new();
            print!("Choose an apartment number: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).ok();
            match input.trim().parse::<usize>() {
                Ok(number) => {
                    if available_apartments.contains(&number) && number != own_apartment {
                        return number;
                    } else {
                        println!("No such apartment available.");
                    }
                }
                Err(_) => println!("Invalid input. Please enter a valid apartment number."),
            }
        }
    }
}
