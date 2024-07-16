use crate::hotel::Hotel;
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

    fn set_hotel_id(&mut self, game_flow: &mut game_flow::GameFlow, id: String) {
        match Hotel::upload(&id) {
            Some(hotel) => {
                println!("Hotel found with ID: {}", hotel.id);
                game_flow.hotel = hotel;
            }
            None => {
                println!("Hotel ID set to: {}", id);
                game_flow.hotel.id = id;
            }
        }
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
                self.set_hotel_id(game_flow, input[1].to_string());
            }
            "save" => match game_flow.hotel.save() {
                Ok(_) => {
                    println!("Hotel configuration saved.");
                }
                _ => {
                    println!("Error saving hotel configuration.");
                }
            },
            "rooms" if input.len() == 2 => {
                let prev_num_rooms = game_flow.hotel.num_rooms;
                game_flow.hotel.num_rooms = input[1].parse().unwrap_or(prev_num_rooms);
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
                    game_flow.hotel.reinitialize();
                    println!("Hotel setup complete. Moving to resident settlement stage.");
                    return HandlingResult::ChangeState;
                }
            }
            "help" => {
                println!("Available commands:");
                println!("new -- to reset all info about the hotel and generate a new random ID");
                println!("id [id] -- to upload a hotel with existing ID, or set a new ID to the current hotel");
                println!("save -- to save the configurations of the hotel with the ID");
                println!("rooms [number of rooms] -- to set the number of rooms for the hotel");
                println!(
                    "rps [rooms per story] -- to set the number of rooms per story in the hotel"
                );
                println!("capital [initial capital] -- to set the initial capital for the hotel");
                println!("fee [fee] -- to set the entrance fee for the residents to settle into the hotel");
                println!(
                    "service [daily costs per user] -- to set daily costs of the hotel per user"
                );
                println!("config -- to print the hotel configurations");
                println!("hotel set -- to finish hotel settings and move on to the next stage");
                println!("help -- to get current list of available commands")
            }
            _ => println!("Invalid command. Try 'help' for getting possible options."),
        }
        HandlingResult::KeepState
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::{game_flow::GameFlow, manager::Manager, manager_states::manager_state::ManagerState};

    fn run_commands(manager: &mut Manager, commands: &[&str]) {
        for command in commands {
            let input: Vec<&str> = command.trim().split_whitespace().collect();
            if !input.is_empty() {
                manager.handle_command(&input);
            }
        }
    }

    #[test]
    fn test_set_hotel_id_with_existing_id() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let commands = vec![
            "id 123",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.num_rooms, 123);
        assert_eq!(manager.game_flow.hotel.id, "123");
    }

    #[test]
    fn test_set_hotel_id_with_new_id() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let initial_num_rooms = manager.game_flow.hotel.num_rooms;

        let commands = vec![
            "id new_id",
            ];

        run_commands(&mut manager, &commands);
        
        assert_eq!(manager.game_flow.hotel.num_rooms, initial_num_rooms);
        assert_eq!(manager.game_flow.hotel.id, "new_id");
    }

    #[test]
    fn test_set_number_of_rooms() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let commands = vec![
            "rooms 42",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.num_rooms, 42);
    }

    #[test]
    fn test_set_rooms_per_story() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let commands = vec![
            "rps 10",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.rooms_per_story, 10);
    }

    #[test]
    fn test_set_initial_capital() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let commands = vec![
            "capital 42000",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.capital, 42000.0);
    }

    #[test]
    fn test_set_entrance_fee() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let commands = vec![
            "fee 42",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.entrance_fee, 42.0);
    }

    #[test]
    fn test_set_daily_service_cost() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let commands = vec![
            "service 42",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.daily_costs, 42.0);
    }

    #[test]
    fn test_print_hotel_config() {
        let state = SetUpHotelState;
        let mut game_flow = GameFlow::new();
        game_flow.hotel.id = "test_id".to_string();
        game_flow.hotel.num_rooms = 100;
        game_flow.hotel.rooms_per_story = 10;
        game_flow.hotel.capital = 123456.0;
        game_flow.hotel.entrance_fee = 50.0;
        game_flow.hotel.daily_costs = 200.0;

        state.print_hotel_config(&game_flow.hotel);
    }

    #[test]
    fn test_finalize_hotel_setup() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));

        let commands = vec![
            "rooms 24",
            "hotel set",
            "rooms 42",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.num_rooms, 24);
    }

    #[test]
    fn test_new_hotel_setup() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let initial_num_rooms = manager.game_flow.hotel.num_rooms;

        let commands = vec![
            "rooms 24",
            "new",
        ];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.num_rooms, initial_num_rooms);
        assert_ne!(manager.game_flow.hotel.num_rooms, 24);
    }

    #[test]
    fn test_save_hotel_config() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        let id = manager.game_flow.hotel.id.clone();

        let commands = vec![
            "save"
        ];

        run_commands(&mut manager, &commands);
        
        let path = format!("hotel_configs/{}.json", id);
        assert!(Path::new(&path).exists());
    }

    #[test]
    fn test_help_commands() {
        let mut manager = Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));

        let commands = vec![
            "help",
        ];

        run_commands(&mut manager, &commands);
    }
}
