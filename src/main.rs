mod apartment;
mod debug;
mod document;
mod game_flow;
pub mod game_history;
mod hotel;
pub mod mail;
mod manager;
mod manager_states;
mod resident;
mod roles;
pub mod strategies;
pub mod text_formatters;

use manager::Manager;
use std::io::{self, Write};

fn run() {
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
        if input[0] == "quit" || input[0] == "exit" {
            break;
        }
        manager.handle_command(&input);
    }
}

fn main() {
    // debug::debug();
    run();
}
