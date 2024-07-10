use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::manager::Manager;

#[allow(dead_code)]
pub(crate) fn debug() {
    let file = File::open("inputs/bots_play.txt").unwrap();
    let reader = BufReader::new(file);

    let inputs: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut manager = Manager::new();
    for input in inputs {
        let input: Vec<&str> = input.trim().split_whitespace().collect();
        if input.is_empty() {
            continue;
        }
        manager.handle_command(&input);
    }
}
