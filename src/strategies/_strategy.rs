use crate::{
    hotel,
    resident::{Resident, ResidentType},
    roles::Role,
};
use std::io::{self, Write};

pub trait ResidentStrategy: Send + Sync {
    fn perform_action(&self, resident: &Resident, hotel: &mut hotel::Hotel) {
        match resident.resident_type {
            ResidentType::Human => self.perform_action_human(resident, hotel),
            ResidentType::Bot => self.perform_action_bot(resident, hotel),
        }
    }

    fn perform_action_human(&self, resident: &Resident, hotel: &mut hotel::Hotel);
    fn perform_action_bot(&self, resident: &Resident, hotel: &mut hotel::Hotel);

    fn confess_role(&self) -> Role;

    fn choose_target(&self, hotel: &mut hotel::Hotel) -> usize {
        println!(
            "Available apartments are: {}",
            hotel
                .available_rooms()
                .iter()
                .map(|apt| apt.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        self.get_user_input()
    }

    fn get_user_input(&self) -> usize {
        loop {
            let mut input = String::new();
            print!("Choose an apartment number: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).ok();
            match input.trim().parse::<usize>() {
                Ok(number) => return number,
                Err(_) => println!("Invalid input. Please enter a valid apartment number."),
            }
        }
    }
}
