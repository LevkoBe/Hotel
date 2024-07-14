use std::io;

use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};
use crate::{game_flow, hotel};

#[derive(Clone)]
pub struct PlayingState;

impl PlayingState {
    fn print_hotel(&self, hotel: &hotel::Hotel, style: &str) {
        hotel.print_hotel(style, None, None);
    }

    fn mail(&self, game_flow: &mut game_flow::GameFlow) {
        let resident = game_flow.residents[game_flow.current_moving_player]
            .lock()
            .unwrap();
        let apartment = resident.apartment_number;
        let target = resident
            .strategy
            .choose_target(apartment, &mut game_flow.hotel);
        let mut mail = String::new();
        println!("Write your mail: ");
        io::stdin().read_line(&mut mail).unwrap();
        game_flow.hotel.send_mail(target, mail);
    }
}

impl ManagerStateBehavior for PlayingState {
    fn handle_command(
        &mut self,
        game_flow: &mut game_flow::GameFlow,
        input: &[&str],
    ) -> HandlingResult {
        match input[0] {
            "move" => {
                while !game_flow.next_turn() {
                    // the first human's move will trigger the loop to teminate
                }
            }
            "hotel" if input.len() > 1 => {
                self.print_hotel(&game_flow.hotel, input[1]);
            }
            "mail" => {
                self.mail(game_flow);
            }
            "announce" => {
                game_flow.hotel.announce();
            }
            "clear" => {
                let address = game_flow.current_moving_player;
                game_flow.hotel.apartments[address].clear_mails();
                println!("Your mails were cleared.");
            }
            "cheat" => {
                // Implement cheat logic
                println!("Cheat used");
            }
            "pause" => {
                println!("Game paused");
                return HandlingResult::ChangeState;
            }
            "reveal" => {
                for resident in &game_flow.residents {
                    let resident = resident.lock().unwrap();
                    println!("{}", resident);
                }
            }
            "whoami" => {
                let resident = game_flow.residents[game_flow.current_moving_player]
                    .lock()
                    .unwrap();
                println!("{}", resident);
            }
            "restart" => {
                println!("Game restarted");
                return HandlingResult::ResetState;
            }
            "new" if input.len() > 1 && input[1] == "game" => {
                println!("New game started");
                return HandlingResult::Restart;
            }
            "help" => {
                println!("Available commands:");
                println!("move -- depends on the strategy of the player");
                println!("cheat -- allows to do something unallowed");
                println!("pause -- changes state back to 'game'");
                println!("restart -- restarts the current game");
                println!("new game -- starts a new game");
            }
            _ => println!("Invalid command"),
        }
        HandlingResult::KeepState
    }
}
