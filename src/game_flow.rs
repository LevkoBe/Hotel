use std::sync::Arc;

use crate::{hotel::Hotel, resident::Resident, roles::roles::Role};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct GameFlow {
    pub hotel: Hotel,
    pub current_state: GameTime,
    pub days_passed: usize,
    pub current_moving_player: usize,
    pub flow_sequence: FlowSequence,
    pub residents: Vec<Arc<Resident>>,
}

impl GameFlow {
    pub fn new() -> Self {
        let hotel = Hotel::new(
            "Hotel1".to_string(),
            16,
            10000.0,
            crate::hotel::BuildingType::Rectangular,
            2,
            4,
            1000.0,
            200.0,
        );
        Self {
            hotel,
            current_state: GameTime::Day,
            days_passed: 0,
            current_moving_player: 0,
            flow_sequence: FlowSequence::Ordered,
            residents: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.residents = self.hotel.get_all_residents();
        match self.flow_sequence {
            FlowSequence::Alphabetical => {
                self.residents.sort_by(|a, b| a.name.cmp(&b.name));
            }
            FlowSequence::Ordered => {
                let possible_roles: Vec<Role> = Role::iter().collect();
                self.residents.sort_by(|a, b| {
                    let a_role_index = possible_roles
                        .iter()
                        .position(|r| *r == a.strategy.confess_role())
                        .unwrap();
                    let b_role_index = possible_roles
                        .iter()
                        .position(|r| *r == b.strategy.confess_role())
                        .unwrap();
                    a_role_index.cmp(&b_role_index)
                });
            }
            FlowSequence::Random => {
                let mut rng = rand::thread_rng();
                self.residents.shuffle(&mut rng);
            }
            FlowSequence::Chaotic => {
                // Sorting each move*
            }
        }
    }

    pub fn next_turn(&mut self) {
        let cur_player = &mut self.residents[self.current_moving_player];
        cur_player.perform_action(&mut self.hotel);
        self.current_moving_player = (self.current_moving_player + 1) % self.residents.len();
        if self.current_moving_player == 0 {
            self.switch_day_night();
        }
    }

    pub fn switch_day_night(&mut self) {
        self.check_win_lose();
        match self.current_state {
            GameTime::Day => {
                self.current_state = GameTime::Night;
                match self.flow_sequence {
                    FlowSequence::Chaotic => {
                        let mut rng = rand::thread_rng();
                        self.residents.shuffle(&mut rng);
                    }
                    _ => {}
                }
                println!("It's night time!");
            }
            GameTime::Night => {
                self.days_passed += 1;
                self.current_state = GameTime::Day;
                println!("It's day time!");
                self.daily_announcement();
            }
        }
    }

    pub fn check_win_lose(&self) -> bool {
        // Check win/lose logic
        false
    }

    pub fn daily_announcement(&self) -> String {
        // Daily announcement logic
        "Daily announcement".to_string()
    }
}

#[derive(Clone, Copy)]
pub enum GameTime {
    Day,
    Night,
}

#[derive(EnumIter, Clone, Copy, Debug)]
pub enum FlowSequence {
    Ordered,
    Random,
    Alphabetical,
    Chaotic,
}
