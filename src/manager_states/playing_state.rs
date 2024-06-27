use crate::manager::{Manager, ManagerState};
use super::manager_state_behavior::ManagerStateBehavior;

#[derive(Clone)]
pub struct PlayingState;

impl ManagerStateBehavior for PlayingState {
    fn handle_command(&mut self, manager: &mut Manager, input: &[&str]) {
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
                manager.set_state(ManagerState::Game(Box::new(crate::manager_states::GameState)));
            }
            "help" => {
                println!("Available commands:");
                println!("move -- depends on the strategy of the player");
                println!("cheat -- allows to do something unallowed");
                println!("pause -- changes state back to 'game'");
            }
            _ => println!("Invalid command"),
        }
    }
}
