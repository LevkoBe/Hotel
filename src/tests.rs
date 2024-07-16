#[cfg(test)]
mod tests {
    use crate::manager::Manager;

    fn run_commands(manager: &mut Manager, commands: &[&str]) {
        for command in commands {
            let input: Vec<&str> = command.trim().split_whitespace().collect();
            if !input.is_empty() {
                manager.handle_command(&input);
            }
        }
    }

    #[test]
    fn test_initialize_hotel_with_valid_parameters() {
        let mut manager = Manager::new();
        let commands = vec![
            "id unique969",
            "rooms 99",
            "capital 99999999",
            "rps 9",
            "hotel set",
        ];
        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.id, "unique969");
        assert_eq!(manager.game_flow.hotel.num_rooms, 99);
        assert_eq!(manager.game_flow.hotel.capital, 99999999.0);
        assert_eq!(manager.game_flow.hotel.rooms_per_story, 9);
    }

    #[test]
    fn test_initialize_hotel_with_invalid_parameters() {
        let mut manager = Manager::new();
        let initial_num_rooms = manager.game_flow.hotel.num_rooms;
        let commands = vec!["id unique969", "rooms many", "hotel set"];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.id, "unique969");
        assert_eq!(manager.game_flow.hotel.num_rooms, initial_num_rooms);
    }

    #[test]
    fn test_upload_hotel_info_with_valid_id() {
        let mut manager = Manager::new();
        let commands = vec!["rooms 99", "id 123"];
        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.id, "123");
        assert_eq!(manager.game_flow.hotel.num_rooms, 123);
    }

    #[test]
    fn test_upload_hotel_info_with_nonexisting_id() {
        let mut manager = Manager::new();
        let commands = vec!["rooms 99", "id nonexisting"];
        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.hotel.id, "nonexisting");
        assert_eq!(manager.game_flow.hotel.num_rooms, 99);
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
}
