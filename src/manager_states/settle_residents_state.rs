use crate::manager::{Manager, ManagerState};

use super::manager_state_behavior::ManagerStateBehavior;

#[derive(Clone)]
pub struct SettleResidentsState;

impl ManagerStateBehavior for SettleResidentsState {
    fn handle_command(&mut self, manager: &mut Manager, input: &[&str]) {
        match input[0] {
            "add" if input.len() == 6 && input[1] == "resident" => {
                if let Some(ref mut _hotel) = manager.hotel {
                    let name = input[2].to_string();
                    let age: usize = input[3].parse().unwrap_or(0);
                    let account_balance: f64 = input[4].parse().unwrap_or(0.0);
                    let apartment_number: usize = input[5].parse().unwrap_or(0);
                    manager.add_resident(name, age, account_balance, Some(apartment_number));
                } else {
                    println!("Hotel is not set up. Please set up the hotel first.");
                }
            }
            "get" if input.len() == 3 && input[1] == "room" => {
                if let Some(ref hotel) = manager.hotel {
                    let apartment_number: usize = input[2].parse().unwrap_or(0);
                    match hotel.get_room(apartment_number) {
                        Some((number, floor)) => println!("Room {} on floor {}", number, floor),
                        None => println!("Apartment not found"),
                    }
                } else {
                    println!("Hotel is not set up. Please set up the hotel first.");
                }
            }
            "residents" if input.len() == 2 && input[1] == "settled" => {
                if let Some(_) = manager.hotel {
                    manager.settle_remaining_residents();
                    println!("Residents settled. Moving to game stage.");
                    manager.set_state(ManagerState::Game(Box::new(crate::manager_states::GameState)));
                } else {
                    println!("Hotel is not set up. Please set up the hotel first.");
                }
            }
            "help" => {
                println!("Available commands:");
                println!("add resident [name] [age] [account balance] [apartment] -- to add a resident to the hotel");
                println!("get room [apartment number] -- to get the floor and room number of the apartment");
                println!("residents settled -- to move on to the next stage");
            }
            _ => println!("Invalid command"),
        }
    }
}
