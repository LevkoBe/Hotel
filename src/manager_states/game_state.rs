use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::{game_flow, hotel};

#[derive(Clone)]
pub struct GameState;

impl ManagerStateBehavior for GameState {
    fn finish_setting(&self) -> hotel::Hotel {
        todo!()
    }
    
    fn handle_command (
        &mut self,
        game_flow: &mut Option<game_flow::GameFlow>,
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
            "play" => {
                println!("Game started");
                return HandlingResult::ChangeState;
            }
            "save" => {
                if let Some(ref game_flow) = game_flow {
                    // todo: Implement save logic here
                    println!("Game progress saved for hotel ID: {}", game_flow.hotel.id);
                } else {
                    println!("Hotel is not set up. Cannot save game progress.");
                }
            }
            _ => println!("Invalid command"),
        }
        HandlingResult::KeepState
    }
}
