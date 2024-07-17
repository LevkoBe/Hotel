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

        if let Some(apartment_number) = apartment_number {
            if hotel.is_room_available(apartment_number) {
                if let Some(role) = hotel.random_available_role() {
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
                    println!("No roles available");
                }
            } else {
                println!("Room unavailable");
            }
        } else {
            println!("No apartment number provided");
            return;
        }
    }

    pub fn settle_remaining_residents(&self, hotel: &mut hotel::Hotel) {
        while hotel.available_rooms_count() > 0 {
            if let Some(next_available_room) = hotel.find_next_available_room() {
                if let Some(role) = hotel.random_available_role() {
                    let bot = ResidentFactory::generate_random(next_available_room, role);
                    hotel.add_resident(bot, next_available_room);
                }
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
                println!("Available rooms: {:?}", game_flow.hotel.available_rooms());
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
                println!("available -- to get the list of available rooms");
                println!("residents settled -- to move on to the next stage");
            }
            _ => println!("Invalid command. Try 'help' instead."),
        }
        HandlingResult::KeepState
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        manager::Manager,
        manager_states::{manager_state::ManagerState, SetUpHotelState},
    };

    fn run_commands(manager: &mut Manager, commands: &[&str]) {
        for command in commands {
            let input: Vec<&str> = command.trim().split_whitespace().collect();
            if !input.is_empty() {
                manager.handle_command(&input);
            }
        }
    }

    #[test]
    fn test_add_resident_with_available_room() {
        let mut manager = Manager::new_with_state(ManagerState::SettleResidents(Box::new(
            SettleResidentsState,
        )));
        manager.game_flow.hotel.num_rooms = 10;

        let commands = vec![
            "add resident [name] [age] [account balance] [apartment]", // this line won't affect the result
            "add resident John 30 1000 1",
        ];

        run_commands(&mut manager, &commands);

        if let Some(resident) = &manager.game_flow.hotel.apartments[1].resident {
            let resident = resident.lock().unwrap();

            assert_eq!(resident.name, "John");
            assert_eq!(resident.age, 30);
            assert_eq!(resident.account_balance, 1000.0);
        } else {
            panic!("Resident not found in the apartment!");
        }
    }

    #[test]
    fn test_add_resident_with_unavailable_room() {
        let mut manager = Manager::new_with_state(ManagerState::SettleResidents(Box::new(
            SettleResidentsState,
        )));

        let commands = vec![
            "add resident [name] [age] [account balance] [apartment]", // this line won't affect the result
            "add resident John 30 1000 1",
            "add resident Hannah 28 2000 1",
        ];

        run_commands(&mut manager, &commands);

        if let Some(resident) = &manager.game_flow.hotel.apartments[1].resident {
            let resident = resident.lock().unwrap();

            assert_eq!(resident.name, "John");
            assert_eq!(resident.age, 30);
            assert_eq!(resident.account_balance, 1000.0);
        } else {
            panic!("Resident not found in the apartment!");
        }
    }

    #[test]
    fn test_add_resident_with_no_apartment_number() {
        let mut manager = Manager::new_with_state(ManagerState::SettleResidents(Box::new(
            SettleResidentsState,
        )));
        manager.game_flow.hotel.num_rooms = 10;

        let commands = vec!["add resident John 30 1000"];

        run_commands(&mut manager, &commands);

        let resident = &manager.game_flow.hotel.apartments[1].resident;
        assert!(resident.is_none());
    }

    #[test]
    fn test_add_resident_with_no_rooms_available() {
        let mut manager =
            Manager::new_with_state(ManagerState::SetUpHotel(Box::new(SetUpHotelState)));
        manager.game_flow.hotel.num_rooms = 0;

        let commands = vec!["rooms 0", "hotel set", "add resident John 30 1000 1"];

        run_commands(&mut manager, &commands);

        let resident = &manager.game_flow.hotel.apartments[1].resident;
        assert!(resident.is_none());
    }

    #[test]
    fn test_settle_remaining_residents() {
        let mut manager = Manager::new_with_state(ManagerState::SettleResidents(Box::new(
            SettleResidentsState,
        )));
        let initial_num_rooms = manager.game_flow.hotel.num_rooms;

        let commands = vec!["residents settled"];

        run_commands(&mut manager, &commands);

        assert!(initial_num_rooms > 0);
        assert_eq!(manager.game_flow.hotel.available_rooms_count(), 0);
    }

    #[test]
    fn test_command_to_retrieve_floor_and_room_number() {
        let mut manager = Manager::new();
        let commands = vec!["hotel set", "get room 13"];
        run_commands(&mut manager, &commands);

        let floor_and_room = manager.game_flow.hotel.get_room(13);

        if let Some((floor, room)) = floor_and_room {
            assert_eq!(floor, 1); // 2nd floor
            assert_eq!(room, 3); // 4th in the corridor
        } else {
            panic!("Expected Some((floor, room)) but got None");
        }
    }

    #[test]
    fn test_help_commands() {
        let mut manager = Manager::new_with_state(ManagerState::SettleResidents(Box::new(
            SettleResidentsState,
        )));

        let commands = vec!["help"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }
}
