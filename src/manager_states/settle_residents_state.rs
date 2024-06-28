use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::{hotel::{self, Hotel}, resident::ResidentFactory, roles::Role};

#[derive(Clone)]
pub struct SettleResidentsState;

impl SettleResidentsState {
    pub fn add_resident(
        &self,
        hotel: &mut Option<hotel::Hotel>,
        name: String,
        age: usize,
        account_balance: f64,
        apartment_number: Option<usize>,
    ) {
        if hotel.as_ref().unwrap().available_rooms() == 0 {
            println!("No rooms available");
            return;
        }

        let hotel = hotel.as_mut().unwrap();
        if let Some(role) = hotel.random_available_role() {
            let resident = ResidentFactory::create_resident(name, age, account_balance, role);
            if let Some(apartment_number) = apartment_number {
                if hotel.is_room_available(apartment_number) {
                    hotel.add_resident(resident, apartment_number);
                } else {
                    println!("Room unavailable");
                }
            } else {
                println!("No apartment number provided");
            }
        } else {
            println!("No roles available");
            return;
        }
    }

    pub fn settle_remaining_residents(&self, hotel: &mut Option<hotel::Hotel>) {
        if let Some(ref mut hotel) = hotel {
            while hotel.available_rooms() > 0 {
                let bot = ResidentFactory::generate_random();
                if let Some(next_available_room) = hotel.find_next_available_room() {
                    hotel.add_resident(bot, next_available_room);
                } else {
                    break;
                }
            }
            println!("Remaining rooms settled with bots");
        } else {
            println!("Hotel is not set up. Please set up the hotel first.");
        }
    }
}

impl ManagerStateBehavior for SettleResidentsState {
    fn finish_setting(&self, hotel: Option<hotel::Hotel>) -> hotel::Hotel {
        hotel.unwrap_or_else(|| {
            panic!("Hotel is not set up. Cannot finish setting up the game state.");
            // todo
        })
    }

    fn handle_command(
        &mut self,
        hotel: &mut Option<hotel::Hotel>,
        input: &[&str],
    ) -> HandlingResult {
        match input[0] {
            "add" if input.len() == 6 && input[1] == "resident" => {
                if let Some(ref mut _hotel) = hotel {
                    let name = input[2].to_string();
                    let age: usize = input[3].parse().unwrap_or(0);
                    let account_balance: f64 = input[4].parse().unwrap_or(0.0);
                    let apartment_number: usize = input[5].parse().unwrap_or(0);
                    self.add_resident(hotel, name, age, account_balance, Some(apartment_number));
                } else {
                    println!("Hotel is not set up. Please set up the hotel first.");
                }
            }
            "available" => {
                if let Some(ref hotel) = hotel {
                    println!("Available rooms: {}", hotel.available_rooms());
                }
            }
            "get" if input.len() == 3 && input[1] == "room" => {
                if let Some(ref hotel) = hotel {
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
                if let Some(_) = hotel {
                    self.settle_remaining_residents(hotel);
                    println!("Residents settled. Moving to game stage.");
                    return HandlingResult::ChangeState;
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
        HandlingResult::KeepState
    }
}
