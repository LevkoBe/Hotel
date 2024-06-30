use std::sync::{Arc, Mutex};

use crate::{
    hotel::Hotel,
    resident::{Resident, ResidentType},
    roles::Role,
};
use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct GameFlow {
    pub hotel: Hotel,
    pub current_state: GameTime,
    pub days_passed: usize,
    pub current_moving_player: usize,
    pub flow_sequence: FlowSequence,
    pub residents: Vec<Arc<Mutex<Resident>>>,
}

impl GameFlow {
    pub fn new() -> Self {
        let random_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let hotel = Hotel::new(
            random_id,
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
                self.residents.sort_by(|a, b| {
                    let a_name = a.lock().unwrap().name.clone();
                    let b_name = b.lock().unwrap().name.clone();
                    a_name.cmp(&b_name)
                });
            }
            FlowSequence::Ordered => {
                let possible_roles: Vec<Role> = Role::iter().collect();
                self.residents.sort_by(|a, b| {
                    let a_role_index = {
                        let a_resident = a.lock().unwrap();
                        possible_roles
                            .iter()
                            .position(|r| *r == a_resident.strategy.confess_role())
                            .unwrap()
                    };
                    let b_role_index = {
                        let b_resident = b.lock().unwrap();
                        possible_roles
                            .iter()
                            .position(|r| *r == b_resident.strategy.confess_role())
                            .unwrap()
                    };
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
            FlowSequence::Scheduled => {
                unimplemented!()
                // more like an online-version game, with each person scheduling their night's walk at specific time
            }
        }
    }

    pub fn next_turn(&mut self) -> bool {
        // bool(next *human* turn made)
        self.current_state = GameTime::Night;
        let is_human;
        {
            let cur_player = &self.residents[self.current_moving_player];
            let resident = cur_player.lock().unwrap();
            resident.perform_action(&mut self.hotel);
            is_human = resident.resident_type == ResidentType::Human;
        }
        self.current_moving_player = (self.current_moving_player + 1) % self.residents.len();
        if self.current_moving_player == 0 {
            self.switch_day_night();
            return true; // even if all moves were done by bots, time to stop
        }
        is_human
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
    Scheduled,
}
