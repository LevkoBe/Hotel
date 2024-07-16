use std::io;

use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::{game_flow, hotel};

#[derive(Clone)]
pub struct PlayingState;

impl PlayingState {
    fn print_hotel(&self, hotel: &hotel::Hotel, style: &str) {
        hotel.print_hotel(style, None, None);
    }

    fn mail(&self, game_flow: &mut game_flow::GameFlow) {
        let resident = game_flow.residents[game_flow.current_moving_player]
            .lock()
            .unwrap();
        let apartment = resident.apartment_number;
        let target = resident
            .strategy
            .choose_target(apartment, &mut game_flow.hotel);
        let mut mail = String::new();
        println!("Write your mail: ");
        io::stdin().read_line(&mut mail).unwrap();
        game_flow.hotel.send_mail(target, mail);
    }
}

impl ManagerStateBehavior for PlayingState {
    fn handle_command(
        &mut self,
        game_flow: &mut game_flow::GameFlow,
        input: &[&str],
    ) -> HandlingResult {
        match input[0] {
            "move" => {
                while !game_flow.next_turn() {
                    // the first human's move will trigger the loop to teminate
                }
            }
            "hotel" if input.len() > 1 => {
                self.print_hotel(&game_flow.hotel, input[1]);
            }
            "mail" => {
                self.mail(game_flow);
            }
            "announce" => {
                game_flow.hotel.announce();
            }
            "clear" => {
                let player_number = game_flow.current_moving_player;
                let address = game_flow.residents[player_number]
                    .lock()
                    .unwrap()
                    .apartment_number;
                game_flow.hotel.apartments[address].clear_mails();
                println!("Your mails were cleared.");
            }
            "cheat" => {
                // Implement cheat logic
                println!("Cheat used");
            }
            "pause" => {
                println!("Game paused");
                return HandlingResult::ChangeState;
            }
            "reveal" => {
                for resident in &game_flow.residents {
                    let resident = resident.lock().unwrap();
                    println!("{}", resident);
                }
            }
            "whoami" => {
                let resident = game_flow.residents[game_flow.current_moving_player]
                    .lock()
                    .unwrap();
                println!("{}", resident);
            }
            "restart" => {
                println!("Game restarted");
                return HandlingResult::Restart;
            }
            "new" if input.len() > 1 && input[1] == "game" => {
                println!("New game started");
                return HandlingResult::ResetState;
            }
            "help" => {
                println!("Available commands:");
                println!("move -- depends on the strategy of the player");
                println!("cheat -- allows to do something unallowed");
                println!("pause -- changes state back to 'game'");
                println!("restart -- restarts the current game");
                println!("new -- starts a new game");
            }
            _ => println!("Invalid command"),
        }
        HandlingResult::KeepState
    }
}

#[cfg(test)]
mod tests {
    use crate::{manager::Manager, manager_states::manager_state::ManagerState};

    use super::*;

    fn run_commands(manager: &mut Manager, commands: &[&str]) {
        for command in commands {
            let input: Vec<&str> = command.trim().split_whitespace().collect();
            if !input.is_empty() {
                manager.handle_command(&input);
            }
        }
    }

    #[test]
    fn test_move_command() {
        let mut manager = Manager::new();
        let commands = vec!["hotel set", "residents settled", "play", "move"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }

    #[test]
    fn test_hotel_command() {
        let mut manager = Manager::new_with_state(ManagerState::Playing(Box::new(PlayingState)));
        let commands = vec!["hotel default"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }

    #[test]
    fn test_clear_command() {
        let mut manager = Manager::new();
        let commands = vec!["hotel set", "residents settled", "play", "clear"];

        run_commands(&mut manager, &commands);

        let player_number = manager.game_flow.current_moving_player;
        let address = manager.game_flow.residents[player_number]
            .lock()
            .unwrap()
            .apartment_number;

        assert_eq!(manager.game_flow.hotel.apartments[address].mails.len(), 0);
    }

    #[test]
    fn test_cheat_command() {
        let mut manager = Manager::new_with_state(ManagerState::Playing(Box::new(PlayingState)));
        let commands = vec!["cheat"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }

    #[test]
    fn test_pause_command() {
        let mut manager = Manager::new_with_state(ManagerState::Playing(Box::new(PlayingState)));
        let commands = vec!["pause", "move"];

        run_commands(&mut manager, &commands);
    }

    #[test]
    fn test_reveal_command() {
        let mut manager = Manager::new_with_state(ManagerState::Playing(Box::new(PlayingState)));
        let commands = vec!["reveal"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }

    #[test]
    fn test_whoami_command() {
        let mut manager = Manager::new();
        let commands = vec!["hotel set", "residents settled", "play", "whoami"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }

    #[test]
    fn test_restart_command() {
        let mut manager = Manager::new_with_state(ManagerState::Playing(Box::new(PlayingState)));
        let commands = vec!["restart"];

        run_commands(&mut manager, &commands);
        // The state change will need to be verified manually or with additional mock logic.
    }

    #[test]
    fn test_new_game_command() {
        let mut manager = Manager::new_with_state(ManagerState::Playing(Box::new(PlayingState)));
        let commands = vec!["new"];

        run_commands(&mut manager, &commands);
        // The state change will need to be verified manually or with additional mock logic.
    }

    #[test]
    fn test_help_command() {
        let mut manager = Manager::new_with_state(ManagerState::Playing(Box::new(PlayingState)));
        let commands = vec!["help"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }
}
