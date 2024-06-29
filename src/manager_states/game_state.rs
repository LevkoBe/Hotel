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
