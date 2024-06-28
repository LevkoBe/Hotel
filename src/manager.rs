use crate::manager_states::handling_result::HandlingResult;
use crate::manager_states::manager_state::ManagerState;
use crate::manager_states::{GameState, PlayingState, SetUpHotelState, SettleResidentsState};
use crate::hotel;

pub struct Manager {
    state: ManagerState,
    pub hotel: Option<hotel::Hotel>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            state: ManagerState::SetUpHotel(Box::new(SetUpHotelState::new())),
            hotel: None,
        }
    }

    pub fn save_hotel(&mut self) {
        match &self.state {
            ManagerState::SetUpHotel(state) => {
                self.hotel = Some(state.finish_setting(self.hotel.clone()));
            }
            _ => {}
        }
    }

    pub(crate) fn handle_command(&mut self, input: &[&str]) {
        let result = self.state.handle_command(&mut self.hotel, input);
        match result {
            HandlingResult::KeepState => {}
            HandlingResult::ResetState => match self.state {
                ManagerState::SetUpHotel(_) => {
                    self.hotel = None;
                    self.state = ManagerState::SetUpHotel(Box::new(SetUpHotelState::new()));
                }
                ManagerState::SettleResidents(_) => {
                    self.state = ManagerState::SettleResidents(Box::new(SettleResidentsState));
                }
                ManagerState::Game(_) => {
                    self.state = ManagerState::Game(Box::new(GameState));
                }
                ManagerState::Playing(_) => {
                    self.state = ManagerState::Game(Box::new(PlayingState));
                }
            },
            HandlingResult::ChangeState => match self.state {
                ManagerState::SetUpHotel(_) => {
                    self.save_hotel();
                    self.state = ManagerState::SettleResidents(Box::new(SettleResidentsState));
                }
                ManagerState::SettleResidents(_) => {
                    self.state = ManagerState::Game(Box::new(GameState));
                }
                ManagerState::Game(_) => {
                    self.state = ManagerState::SetUpHotel(Box::new(PlayingState));
                }
                ManagerState::Playing(_) => {
                    self.state = ManagerState::SetUpHotel(Box::new(GameState));
                }
            },
            HandlingResult::Restart => {
                self.state = ManagerState::SetUpHotel(Box::new(SetUpHotelState::new()));
            },
        }
    }
}
