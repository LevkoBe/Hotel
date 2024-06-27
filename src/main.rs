mod hotel;
mod apartment;
mod resident;
mod document;
mod game_flow;
mod roles;
mod patterns;

use game_flow::GameFlowSingleton;
use resident::{ResidentFactory, Role};

fn main() {
    // Initialize hotel with some basic configuration
    let mut hotel = hotel::Hotel::new(
        "Hotel1".to_string(),
        10,
        10000.0,
        hotel::BuildingType::Rectangular,
    );

    // Add apartments
    for i in 0..10 {
        hotel.add_apartment(apartment::Apartment::new(i, i / 2));
    }

    // Create residents using factory pattern
    let residents = vec![
        ResidentFactory::create_resident("Alice".to_string(), 30, 100.0, Role::Killer),
        ResidentFactory::create_resident("Bob".to_string(), 25, 150.0, Role::Policeman),
    ];

    // Assign residents to apartments
    hotel.apartments[0].assign_resident(residents[0].clone());
    hotel.apartments[1].assign_resident(residents[1].clone());

    // Initialize game flow singleton
    let game_flow = GameFlowSingleton::get_instance();
    game_flow.lock().unwrap().initialize(residents);

    // Start the game loop
    loop {
        let mut game_flow = game_flow.lock().unwrap();
        game_flow.execute();
        if game_flow.check_win_lose() {
            break;
        }
    }

    println!("Game Over!");
}
