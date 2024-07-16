use crate::{
    game_flow::GameFlow,
    hotel::Hotel,
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
            state: ManagerState::SetUpHotel(Box::new(SetUpHotelState)),
            game_flow: GameFlow::new(),
        }
    }

    pub fn new_with_state(state: ManagerState) -> Self {
        Manager {
            state,
            game_flow: GameFlow::new(),
        }
    }

    pub fn empty_hotel(&mut self) {
        let prev_id = self.game_flow.hotel.id.clone();
        let prev_num_rooms = self.game_flow.hotel.num_rooms;
        let prev_capital = self.game_flow.hotel.capital;
        let prev_building_type = self.game_flow.hotel.building_type;
        let prev_elevator_position = self.game_flow.hotel.elevator_position;
        let prev_rooms_per_story = self.game_flow.hotel.rooms_per_story;
        let prev_entrance_fee = self.game_flow.hotel.entrance_fee;
        let prev_daily_costs = self.game_flow.hotel.daily_costs;

        self.game_flow.hotel = Hotel::new(
            prev_id,
            prev_num_rooms,
            prev_capital,
            prev_building_type,
            prev_elevator_position,
            prev_rooms_per_story,
            prev_entrance_fee,
            prev_daily_costs,
        );
    }

    pub(crate) fn handle_command(&mut self, input: &[&str]) {
        let result = self.state.handle_command(&mut self.game_flow, input);
        match result {
            HandlingResult::KeepState => {}
            HandlingResult::ResetState => match self.state {
                ManagerState::SetUpHotel(_) => {
                    self.game_flow = GameFlow::new();
                    self.state = ManagerState::SetUpHotel(Box::new(SetUpHotelState));
                }
                ManagerState::SettleResidents(_) => {
                    self.empty_hotel();
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
                self.state = ManagerState::SetUpHotel(Box::new(SetUpHotelState));
            }
        }
    }
}
