use crate::manager_states::handling_result::HandlingResult;
use crate::manager_states::manager_state_behavior::ManagerStateBehavior;
use crate::{hotel, resident::ResidentFactory, roles::Role};
use crate::manager_states::{GameState, SetUpHotelState, SettleResidentsState};

pub enum ManagerState {
    SetUpHotel(Box<dyn ManagerStateBehavior>),
    SettleResidents(Box<dyn ManagerStateBehavior>),
    Game(Box<dyn ManagerStateBehavior>),
}

impl ManagerState {
    fn handle_command(&mut self, hotel: &mut Option<hotel::Hotel>, input: &[&str]) -> HandlingResult {
        match self {
            ManagerState::SetUpHotel(state) => state.handle_command(hotel, input),
            ManagerState::SettleResidents(state) => state.handle_command(hotel, input),
            ManagerState::Game(state) => state.handle_command(hotel, input),
        }
    }
}

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
            HandlingResult::ResetState => {
                match self.state {
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
                }
            }
            HandlingResult::ChangeState => {
                match self.state {
                    ManagerState::SetUpHotel(_) => {
                        self.save_hotel();
                        self.state = ManagerState::SettleResidents(Box::new(SettleResidentsState));
                    }
                    ManagerState::SettleResidents(_) => {
                        self.state = ManagerState::Game(Box::new(GameState));
                    }
                    ManagerState::Game(_) => {
                        self.state = ManagerState::SetUpHotel(Box::new(SetUpHotelState::new()));
                    }
                }
            }
        }
    }
}
