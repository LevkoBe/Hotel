use crate::hotel::Hotel;
use crate::resident::Resident;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub struct GameFlow {
    pub hotel: Arc<Hotel>,
    pub current_state: GameTime,
    pub days_passed: usize,
    pub current_moving_player: usize,
    pub flow_sequence: FlowSequence,
    pub residents: Vec<Arc<Resident>>,
}

impl GameFlow {
    pub fn new() -> Self {
        let hotel = Arc::new(Hotel::new(
            "Hotel1".to_string(),
            50,
            100000.0,
            crate::hotel::BuildingType::Rectangular,
            5,
            10,
            100.0,
            20.0,
        ));
        Self {
            hotel,
            current_state: GameTime::Day,
            days_passed: 0,
            current_moving_player: 0,
            flow_sequence: FlowSequence::Ordered,
            residents: Vec::new(),
        }
    }

    pub fn initialize(&mut self, residents: Vec<Arc<Resident>>) {
        self.residents = residents;
    }

    pub fn next_turn(&mut self) {
        // Handle next turn logic
    }

    pub fn switch_day_night(&mut self) {
        // Switch day/night logic
    }

    pub fn check_win_lose(&self) -> bool {
        // Check win/lose logic
        false
    }

    pub fn daily_announcement(&self) -> String {
        // Daily announcement logic
        "Daily announcement".to_string()
    }

    pub fn execute(&mut self) {
        match self.current_state {
            GameTime::Day => self.handle_day(),
            GameTime::Night => self.handle_night(),
        }
    }

    fn handle_day(&mut self) {
        // Day state logic
        self.switch_day_night();
    }

    fn handle_night(&mut self) {
        // Night state logic
        self.switch_day_night();
    }
}

#[derive(Clone, Copy)]
pub enum GameTime {
    Day,
    Night,
}

#[derive(Clone, Copy)]
pub enum FlowSequence {
    Ordered,
    Random,
    Alphabetical,
}

// Singleton pattern for GameFlow
lazy_static! {
    pub static ref GAME_FLOW_SINGLETON: Mutex<GameFlow> = Mutex::new(GameFlow::new());
}

pub struct GameFlowSingleton;

impl GameFlowSingleton {
    pub fn get_instance() -> &'static Mutex<GameFlow> {
        &GAME_FLOW_SINGLETON
    }
}
