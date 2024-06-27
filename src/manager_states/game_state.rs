use crate::manager::{Manager, ManagerState};
use super::manager_state_behavior::ManagerStateBehavior;

#[derive(Clone)]
pub struct GameState;

impl ManagerStateBehavior for GameState {
    fn handle_command(&mut self, manager: &mut Manager, input: &[&str]) {
        match input[0] {
            "help" => {
                println!("Available commands:");
                println!("play -- changes the state into 'playing'");
                println!("save -- saves the progress to a file, using hotel ID as a unique identifier");
            }
            "play" => {
                println!("Game started");
                manager.set_state(ManagerState::Game(Box::new(crate::manager_states::PlayingState)));
            }
            "save" => {
                if let Some(ref hotel) = manager.hotel {
                    // todo: Implement save logic here
                    println!("Game progress saved for hotel ID: {}", hotel.id);
                } else {
                    println!("Hotel is not set up. Cannot save game progress.");
                }
            }
            _ => println!("Invalid command"),
        }
    }
}
