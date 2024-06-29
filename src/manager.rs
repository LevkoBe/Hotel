use crate::{
    game_flow::GameFlow,
    manager_states::{
        handling_result::HandlingResult, manager_state::ManagerState, GameState, PlayingState,
        SetUpHotelState, SettleResidentsState,
    },
};

pub struct Manager {
    state: ManagerState,
    pub game_flow: GameFlow,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            state: ManagerState::SetUpHotel(Box::new(SetUpHotelState::new())),
            game_flow: GameFlow::new(),
        }
    }

    pub(crate) fn handle_command(&mut self, input: &[&str]) {
        let result = self.state.handle_command(&mut self.game_flow, input);
        match result {
            HandlingResult::KeepState => {}
            HandlingResult::ResetState => match self.state {
                ManagerState::SetUpHotel(_) => {
                    self.state = ManagerState::SetUpHotel(Box::new(SetUpHotelState::new()));
                }
                ManagerState::SettleResidents(_) => {
                    self.state = ManagerState::SettleResidents(Box::new(SettleResidentsState));
                }
                ManagerState::Game(_) => {
                    self.state = ManagerState::Game(Box::new(GameState));
                }
                ManagerState::Playing(_) => {
                    self.state = ManagerState::Playing(Box::new(PlayingState));
                }
            },
            HandlingResult::ChangeState => match self.state {
                ManagerState::SetUpHotel(_) => {
                    self.state = ManagerState::SettleResidents(Box::new(SettleResidentsState));
                }
                ManagerState::SettleResidents(_) => {
                    self.state = ManagerState::Game(Box::new(GameState));
                }
                ManagerState::Game(_) => {
                    self.state = ManagerState::Playing(Box::new(PlayingState));
                }
                ManagerState::Playing(_) => {
                    self.state = ManagerState::Game(Box::new(GameState));
                }
            },
            HandlingResult::Restart => {
                self.state = ManagerState::SetUpHotel(Box::new(SetUpHotelState::new()));
            }
        }
    }
}
