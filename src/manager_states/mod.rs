pub mod handling_result;
pub mod manager_state_behavior;

pub mod game_state;
pub mod playing_state;
pub mod settle_residents_state;
pub mod setup_hotel_state;

pub use game_state::GameState;
pub use playing_state::PlayingState;
pub use settle_residents_state::SettleResidentsState;
pub use setup_hotel_state::SetUpHotelState;
