use std::io;

use super::_strategy::ResidentStrategy;
use crate::{
    game_history::{self, GameHistory},
    hotel::Hotel,
    mail::Suspicion,
    resident::Resident,
    roles::Role,
};

pub struct JudgeStrategy;

impl JudgeStrategy {
    fn vote(
        &self,
        suspicion: &mut Suspicion,
        judge_apartment: usize,
        vote_for: bool,
        history: &mut game_history::GameHistory,
    ) {
        if vote_for {
            suspicion.for_votes += 1;
            println!(
                "Judge from apartment {} votes for the arrest of the resident in apartment {}",
                judge_apartment, suspicion.suspected
            );
            history.add_action(
                judge_apartment,
                format!("Vote for {}", suspicion.suspected),
                0,
                None,
            );
        } else {
            suspicion.against_votes += 1;
            println!(
                "Judge from apartment {} votes against the arrest of the resident in apartment {}",
                judge_apartment, suspicion.suspected
            );
            history.add_action(judge_apartment, "Vote against".to_string(), 0, None);
        }
    }
}

impl ResidentStrategy for JudgeStrategy {
    fn perform_action_human(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let judge_apartment = performer.apartment_number;
        for (target, suspicion) in hotel.investigation_queue.iter_mut() {
            println!(
                "Do you vote for the arrest in apartment {}? ('+' if so)",
                target
            );
            let mut vote_input = String::new();
            io::stdin()
                .read_line(&mut vote_input)
                .expect("Failed to read line");

            let vote_for = vote_input.trim().to_lowercase() == "+";
            self.vote(suspicion, judge_apartment, vote_for, history);
            history.add_action(judge_apartment, "Vote".to_string(), 0, None);
        }
    }

    fn perform_action_bot(
        &self,
        performer: &mut Resident,
        hotel: &mut Hotel,
        history: &mut GameHistory,
    ) {
        let judge_apartment = performer.apartment_number;
        for (_, suspicion) in hotel.investigation_queue.iter_mut() {
            let vote_for = rand::random::<f32>() > 0.2; // biased
            self.vote(suspicion, judge_apartment, vote_for, history);
        }
    }

    fn confess_role(&self) -> Role {
        Role::Judge
    }
}
