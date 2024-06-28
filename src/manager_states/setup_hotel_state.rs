use crate::{game_flow, hotel};

use super::handling_result::HandlingResult;
use super::manager_state_behavior::ManagerStateBehavior;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct SetUpHotelState {
    id: String,
    num_rooms: usize,
    rps: usize,
    capital: f64,
    fee: f64,
    service: f64,
}

impl SetUpHotelState {
    pub fn new() -> Self {
        let random_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        SetUpHotelState {
            id: random_id,
            num_rooms: 50,
            rps: 10,
            capital: 100000.0,
            fee: 100.0,
            service: 20.0,
        }
    }

    fn print_hotel_config(&self) {
        println!("Current hotel configuration:");
        println!("ID: {}", self.id);
        println!("Number of rooms: {}", self.num_rooms);
        println!("Rooms per story: {}", self.rps);
        println!("Initial capital: {}", self.capital);
        println!("Entrance fee: {}", self.fee);
        println!("Daily service cost: {}", self.service);
    }
}

impl ManagerStateBehavior for SetUpHotelState {
    fn finish_setting(&self) -> hotel::Hotel {
        hotel::Hotel::new(
            self.id.clone(),
            self.num_rooms,
            self.capital,
            hotel::BuildingType::Rectangular,
            0,
            self.rps,
            self.fee,
            self.service,
        )
    }
    fn handle_command(
        &mut self,
        _: &mut Option<game_flow::GameFlow>,
        input: &[&str],
    ) -> HandlingResult {
        match input[0] {
            "new" => {
                println!("Hotel reset with a new random ID.");
                return HandlingResult::ResetState;
            }
            "id" if input.len() == 2 => {
                self.id = input[1].to_string();
                println!("Hotel ID set to {}", input[1]);
            }
            "rooms" if input.len() == 2 => {
                self.num_rooms = input[1].parse().unwrap_or(0);
                println!("Number of rooms set to {}", input[1]);
            }
            "rps" if input.len() == 2 => {
                self.rps = input[1].parse().unwrap_or(0);
                println!("Rooms per story set to {}", input[1]);
            }
            "capital" if input.len() == 2 => {
                self.capital = input[1].parse().unwrap_or(0.0);
                println!("Initial capital set to {}", input[1]);
            }
            "fee" if input.len() == 2 => {
                self.fee = input[1].parse().unwrap_or(0.0);
                println!("Entrance fee set to {}", input[1]);
            }
            "service" if input.len() == 2 => {
                self.service = input[1].parse().unwrap_or(0.0);
                println!("Daily service cost set to {}", input[1]);
            }
            "config" => {
                self.print_hotel_config();
            }
            "hotel" if input.len() >= 2 && input[1] == "set" => {
                if self.id.is_empty()
                    || self.num_rooms == 0
                    || self.capital == 0.0
                    || self.fee == 0.0
                    || self.service == 0.0
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
