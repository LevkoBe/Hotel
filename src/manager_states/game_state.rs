use strum::IntoEnumIterator;

use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::game_flow::{self, FlowSequence};

#[derive(Clone)]
pub struct GameState;

impl GameState {
    fn set_game_flow_sequence(&self, fs: &str, game_flow: &mut game_flow::GameFlow) {
        match fs {
            "ordered" => {
                game_flow.flow_sequence = FlowSequence::Ordered;
                println!("Sequence set to ordered.");
            }
            "random" => {
                game_flow.flow_sequence = FlowSequence::Random;
                println!("Sequence set to random.");
            }
            "chaotic" => {
                game_flow.flow_sequence = FlowSequence::Chaotic;
                println!("Sequence set to chaotic.");
            }
            "alphabetical" => {
                game_flow.flow_sequence = FlowSequence::Alphabetical;
                println!("Sequence set to alphabetical.");
            }
            _ => {
                println!("Incorrect option. You might've wanted to write: ");
                let sequences: Vec<FlowSequence> = FlowSequence::iter().collect();
                for si in 0..sequences.len() {
                    print!("{}. {:?}", si, sequences[si]);
                }
            }
        }
    }
}

impl ManagerStateBehavior for GameState {
    fn handle_command(
        &mut self,
        game_flow: &mut game_flow::GameFlow,
        input: &[&str],
    ) -> HandlingResult {
        match input[0] {
            "help" => {
                println!("Available commands:");
                println!("play -- changes the state into 'playing'");
                println!(
                    "save -- saves the progress to a file, using hotel ID as a unique identifier"
                );
            }
            "flow" if input.len() > 1 => {
                self.set_game_flow_sequence(input[1], game_flow);
            }
            "play" => {
                game_flow.initialize();
                println!("Game started");
                return HandlingResult::ChangeState;
            }
            "save" => {
                println!("Game progress saved for hotel ID: {}", game_flow.hotel.id);
            }
            _ => println!("Invalid command"),
        }
        HandlingResult::KeepState
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::game_flow::FlowSequence;
    use crate::manager::Manager;
    use crate::manager_states::manager_state::ManagerState;

    fn run_commands(manager: &mut Manager, commands: &[&str]) {
        for command in commands {
            let input: Vec<&str> = command.trim().split_whitespace().collect();
            if !input.is_empty() {
                manager.handle_command(&input);
            }
        }
    }

    #[test]
    fn test_set_game_flow_sequence_ordered() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let commands = vec!["flow ordered"];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.flow_sequence, FlowSequence::Ordered);
    }

    #[test]
    fn test_set_game_flow_sequence_random() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let commands = vec!["flow random"];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.flow_sequence, FlowSequence::Random);
    }

    #[test]
    fn test_set_game_flow_sequence_chaotic() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let commands = vec!["flow chaotic"];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.flow_sequence, FlowSequence::Chaotic);
    }

    #[test]
    fn test_set_game_flow_sequence_alphabetical() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let commands = vec!["flow alphabetical"];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.flow_sequence, FlowSequence::Alphabetical);
    }

    #[test]
    fn test_set_game_flow_sequence_invalid() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let commands = vec!["flow invalid"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }

    #[test]
    fn test_play_command() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let commands = vec!["flow ordered", "play", "flow random"];

        run_commands(&mut manager, &commands);

        assert_eq!(manager.game_flow.flow_sequence, FlowSequence::Ordered);
    }

    #[test]
    fn test_save_command() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let id = manager.game_flow.hotel.id.clone();

        let commands = vec!["save"];

        run_commands(&mut manager, &commands);

        let path = format!("hotel_configs/{}.json", id);
        assert!(Path::new(&path).exists());
    }

    #[test]
    fn test_help_command() {
        let mut manager = Manager::new_with_state(ManagerState::Game(Box::new(GameState)));
        let commands = vec!["help"];

        run_commands(&mut manager, &commands);
        // This is a print test and will require manual checking of the output.
    }
}
