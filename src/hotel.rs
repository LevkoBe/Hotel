use crate::apartment::Apartment;
use crate::patterns::singleton::Singleton;
use std::sync::Mutex;

pub struct Hotel {
    id: String,
    num_rooms: usize,
    capital: f64,
    building_type: BuildingType,
    pub apartments: Vec<Apartment>,
}

impl Hotel {
    pub fn new(id: String, num_rooms: usize, capital: f64, building_type: BuildingType) -> Self {
        Self {
            id,
            num_rooms,
            capital,
            building_type,
            apartments: Vec::new(),
        }
    }

    pub fn add_apartment(&mut self, apartment: Apartment) {
        self.apartments.push(apartment);
    }

    pub fn get_room(&self, apartment_number: usize) -> Option<&Apartment> {
        self.apartments.get(apartment_number)
    }
}

#[derive(Clone, Copy)]
pub enum BuildingType {
    Rectangular,
    Pyramidal,
    RandomShaped,
    Lax,
    Custom,
}

// Singleton pattern for the Hotel
lazy_static::lazy_static! {
    pub static ref HOTEL_SINGLETON: Mutex<Singleton<Hotel>> = Mutex::new(Singleton::new());
}
