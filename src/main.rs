mod hotel;
mod apartment;
mod resident;
mod document;
mod game_flow;
mod roles;
mod manager;
pub mod manager_states;

use manager::Manager;
use std::io::{self, Write};


fn main() {
    let mut manager = Manager::new();
    loop {
        print!("    => ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.trim().split_whitespace().collect();
        if input.is_empty() {
            continue;
        }
        manager.handle_command(&input);
    }
}
