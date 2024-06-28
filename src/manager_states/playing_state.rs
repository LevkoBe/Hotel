use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::hotel;

#[derive(Clone)]
pub struct PlayingState;

impl ManagerStateBehavior for PlayingState {
    fn finish_setting(&self, hotel: Option<hotel::Hotel>) -> hotel::Hotel {
        hotel.unwrap_or_else(|| {
            panic!("Hotel is not set up. Cannot finish setting up the game state.");
        })
    }
    fn handle_command(&mut self, _: &mut Option<hotel::Hotel>, input: &[&str]) -> HandlingResult {
        match input[0] {
            "move" => {
                // Implement move logic based on player's strategy
                println!("Player moved");
            }
            "cheat" => {
                // Implement cheat logic
                println!("Cheat used");
            }
            "pause" => {
                println!("Game paused");
                return HandlingResult::ChangeState;
            }
            "help" => {
                println!("Available commands:");
                println!("move -- depends on the strategy of the player");
                println!("cheat -- allows to do something unallowed");
                println!("pause -- changes state back to 'game'");
            }
            _ => println!("Invalid command"),
        }
        HandlingResult::KeepState
    }
}
