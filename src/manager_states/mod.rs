pub mod manager_state_behavior;
pub mod handling_result;

pub mod setup_hotel_state;
pub mod settle_residents_state;
pub mod game_state;
pub mod playing_state;

pub use setup_hotel_state::SetUpHotelState;
pub use settle_residents_state::SettleResidentsState;
pub use game_state::GameState;
pub use playing_state::PlayingState;
