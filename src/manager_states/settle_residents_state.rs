use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::{game_flow, hotel, resident::ResidentFactory};

#[derive(Clone)]
pub struct SettleResidentsState;

impl SettleResidentsState {
    pub fn add_resident(
        &self,
        hotel: &mut hotel::Hotel,
        name: String,
        age: usize,
        account_balance: f64,
        apartment_number: Option<usize>,
    ) {
        if hotel.available_rooms_count() == 0 {
            println!("No rooms available");
            return;
        }

        if let Some(role) = hotel.random_available_role() {
            if let Some(apartment_number) = apartment_number {
                if hotel.is_room_available(apartment_number) {
                    let resident = ResidentFactory::create_resident(
                        name,
                        age,
                        account_balance,
                        apartment_number,
                        role,
                        crate::resident::ResidentType::Human,
                    );
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

    pub fn settle_remaining_residents(&self, hotel: &mut hotel::Hotel) {
        while hotel.available_rooms_count() > 0 {
            if let Some(next_available_room) = hotel.find_next_available_room() {
                let bot = ResidentFactory::generate_random(next_available_room);
                hotel.add_resident(bot, next_available_room);
            } else {
                break;
            }
        }
        println!("Remaining rooms settled with bots");
    }
}

impl ManagerStateBehavior for SettleResidentsState {
    fn handle_command(
        &mut self,
        game_flow: &mut game_flow::GameFlow,
        input: &[&str],
    ) -> HandlingResult {
        match input[0] {
            "add" if input.len() == 6 && input[1] == "resident" => {
                let name = input[2].to_string();
                let age: usize = input[3].parse().unwrap_or(0);
                let account_balance: f64 = input[4].parse().unwrap_or(0.0);
                let apartment_number: usize = input[5].parse().unwrap_or(0);
                self.add_resident(
                    &mut game_flow.hotel,
                    name,
                    age,
                    account_balance,
                    Some(apartment_number),
                );
            }
            "available" => {
                println!(
                    "Available rooms: {}",
                    game_flow.hotel.available_rooms_count()
                );
            }
            "get" if input.len() == 3 && input[1] == "room" => {
                let apartment_number: usize = input[2].parse().unwrap_or(0);
                match game_flow.hotel.get_room(apartment_number) {
                    Some((number, floor)) => println!("Room {} on floor {}", number, floor),
                    None => println!("Apartment not found"),
                }
            }
            "residents" if input.len() == 2 && input[1] == "settled" => {
                self.settle_remaining_residents(&mut game_flow.hotel);
                println!("Residents settled. Moving to game stage.");
                return HandlingResult::ChangeState;
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
