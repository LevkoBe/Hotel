use crate::{game_flow, hotel};

use super::handling_result::HandlingResult;
use super::manager_state_behavior::ManagerStateBehavior;

#[derive(Clone)]
pub struct SetUpHotelState;

impl SetUpHotelState {
    fn print_hotel_config(&self, hotel: &hotel::Hotel) {
        println!("Current hotel configuration:");
        println!("ID: {}", hotel.id);
        println!("Number of rooms: {}", hotel.num_rooms);
        println!("Rooms per story: {}", hotel.rooms_per_story);
        println!("Initial capital: {}", hotel.capital);
        println!("Entrance fee: {}", hotel.entrance_fee);
        println!("Daily service cost: {}", hotel.daily_costs);
    }
}

impl ManagerStateBehavior for SetUpHotelState {
    fn handle_command(
        &mut self,
        game_flow: &mut game_flow::GameFlow,
        input: &[&str],
    ) -> HandlingResult {
        match input[0] {
            "new" => {
                println!("Hotel reset with a new random ID.");
                return HandlingResult::ResetState;
            }
            "id" if input.len() == 2 => {
                // self.id = input[1].to_string();
                game_flow.hotel.id = input[1].to_string();
                println!("Hotel ID set to {}", input[1]);
            }
            "rooms" if input.len() == 2 => {
                game_flow.hotel.num_rooms = input[1].parse().unwrap_or(0);
                println!("Number of rooms set to {}", input[1]);
            }
            "rps" if input.len() == 2 => {
                game_flow.hotel.rooms_per_story = input[1].parse().unwrap_or(0);
                println!("Rooms per story set to {}", input[1]);
            }
            "capital" if input.len() == 2 => {
                game_flow.hotel.capital = input[1].parse().unwrap_or(0.0);
                println!("Initial capital set to {}", input[1]);
            }
            "fee" if input.len() == 2 => {
                game_flow.hotel.entrance_fee = input[1].parse().unwrap_or(0.0);
                println!("Entrance fee set to {}", input[1]);
            }
            "service" if input.len() == 2 => {
                game_flow.hotel.daily_costs = input[1].parse().unwrap_or(0.0);
                println!("Daily service cost set to {}", input[1]);
            }
            "config" => {
                self.print_hotel_config(&game_flow.hotel);
            }
            "hotel" if input.len() >= 2 && input[1] == "set" => {
                if game_flow.hotel.id.is_empty()
                    || game_flow.hotel.num_rooms == 0
                    || game_flow.hotel.capital == 0.0
                    || game_flow.hotel.entrance_fee == 0.0
                    || game_flow.hotel.daily_costs == 0.0
                {
                    println!("Please set all hotel properties before finalizing the setup.");
                } else {
                    println!("Hotel setup complete. Moving to resident settlement stage.");
                    return HandlingResult::ChangeState;
                }
            }
            "help" => {
                println!("Available commands:");
                println!("new -- to reset all info about the hotel and generate a new random ID");
                println!("id [id] -- to set a specific hotel ID");
                println!("rooms [number of rooms] -- to set the number of rooms for the hotel");
                println!(
                    "rps [rooms per story] -- to set the number of rooms per story in the hotel"
                );
                println!("capital [initial capital] -- to set the initial capital for the hotel");
                println!("fee [fee] -- to set the entrance fee for the residents to settle into the hotel");
                println!(
                    "service [daily costs per user] -- to set daily costs of the hotel per user"
                );
                println!("hotel set -- to finish hotel settings and move on to the next stage");
            }
            _ => println!("Invalid command"),
        }
        HandlingResult::KeepState
    }
}
