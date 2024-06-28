use crate::hotel;

use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};


pub enum ManagerState {
    SetUpHotel(Box<dyn ManagerStateBehavior>),
    SettleResidents(Box<dyn ManagerStateBehavior>),
    Game(Box<dyn ManagerStateBehavior>),
    Playing(Box<dyn ManagerStateBehavior>),
}

impl ManagerState {
    pub fn handle_command(
        &mut self,
        hotel: &mut Option<hotel::Hotel>,
        input: &[&str],
    ) -> HandlingResult {
        match self {
            ManagerState::SetUpHotel(state) => state.handle_command(hotel, input),
            ManagerState::SettleResidents(state) => state.handle_command(hotel, input),
            ManagerState::Game(state) => state.handle_command(hotel, input),
            ManagerState::Playing(state) => state.handle_command(hotel, input),
        }
    }
}