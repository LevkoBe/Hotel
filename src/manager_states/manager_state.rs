use crate::game_flow;

use super::{handling_result::HandlingResult, manager_state_behavior::ManagerStateBehavior};

pub enum ManagerState {
    SetUpHotel(Box<dyn ManagerStateBehavior>),
    SettleResidents(Box<dyn ManagerStateBehavior>),
    Game(Box<dyn ManagerStateBehavior>),
    Playing(Box<dyn ManagerStateBehavior>),
}

impl ManagerState {
    pub fn handle_command (
        &mut self,
        game_flow: &mut Option<game_flow::GameFlow>,
        input: &[&str],
    ) -> HandlingResult {
        match self {
            ManagerState::SetUpHotel(state) => state.handle_command(game_flow, input),
            ManagerState::SettleResidents(state) => state.handle_command(game_flow, input),
            ManagerState::Game(state) => state.handle_command(game_flow, input),
            ManagerState::Playing(state) => state.handle_command(game_flow, input),
        }
    }
}
