use crate::manager_states::manager_state_behavior::ManagerStateBehavior;
use crate::{hotel, resident::ResidentFactory, roles::Role};
use crate::manager_states::{GameState, SetUpHotelState, SettleResidentsState};

pub enum ManagerState {
    SetUpHotel(Box<dyn ManagerStateBehavior>),
    SettleResidents(Box<dyn ManagerStateBehavior>),
    Game(Box<dyn ManagerStateBehavior>),
}

impl ManagerState {
    fn handle_command(&mut self, manager: &mut Manager, input: &[&str]) {
        match self {
            ManagerState::SetUpHotel(state) => state.handle_command(manager, input),
            ManagerState::SettleResidents(state) => state.handle_command(manager, input),
            ManagerState::Game(state) => state.handle_command(manager, input),
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

    pub fn set_state(&mut self, state: ManagerState) {
        self.state = state;
    }

    pub(crate) fn handle_command(&mut self, input: &[&str]) {
        let mut state = match self.state {
            ManagerState::SetUpHotel(_) => ManagerState::SetUpHotel(Box::new(SetUpHotelState::new())),
            ManagerState::SettleResidents(_) => ManagerState::SettleResidents(Box::new(SettleResidentsState)),
            ManagerState::Game(_) => ManagerState::Game(Box::new(GameState)),
        };
        state.handle_command(self, input);
    }

    pub fn add_resident(&mut self, name: String, age: usize, account_balance: f64, apartment_number: Option<usize>) {
        if self.hotel.as_ref().unwrap().available_rooms() == 0 {
            println!("No rooms available");
            return;
        }

        let resident = ResidentFactory::create_resident(name, age, account_balance, Role::Newcomer);
        if let Some(apartment_number) = apartment_number {
            self.hotel.as_mut().unwrap().add_resident(resident, apartment_number);
        } else {
            if let Some(next_available_room) = self.hotel.as_mut().unwrap().find_next_available_room() {
                self.hotel.as_mut().unwrap().add_resident(resident, next_available_room);
            } else {
                println!("No rooms available");
            }
        }
    }

    pub fn settle_remaining_residents(&mut self) {
        if let Some(ref mut hotel) = self.hotel {
            while hotel.available_rooms() > 0 {
                let bot = ResidentFactory::generate_random();
                if let Some(next_available_room) = hotel.find_next_available_room() {
                    hotel.add_resident(bot, next_available_room);
                } else {
                    break;
                }
            }
            println!("Remaining rooms settled with bots");
        } else {
            println!("Hotel is not set up. Please set up the hotel first.");
        }
    }
}
