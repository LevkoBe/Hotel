use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::hotel;

#[derive(Clone)]
pub struct GameState;

impl ManagerStateBehavior for GameState {
    fn finish_setting(&self, hotel: Option<hotel::Hotel>) -> hotel::Hotel {
        hotel.unwrap_or_else(|| {
            panic!("Hotel is not set up. Cannot finish setting up the game state.");
        })
    }
    fn handle_command(
        &mut self,
        hotel: &mut Option<hotel::Hotel>,
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
                if let Some(ref hotel) = hotel {
                    // todo: Implement save logic here
                    println!("Game progress saved for hotel ID: {}", hotel.id);
                } else {
                    println!("Hotel is not set up. Cannot save game progress.");
                }
            }
            _ => println!("Invalid command"),
        }
        HandlingResult::KeepState
    }
}
