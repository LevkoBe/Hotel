use std::sync::Arc;

use crate::{apartment::Apartment, resident::Resident};

#[derive(Clone, Copy)]
pub enum BuildingType {
    Rectangular,
    Pyramidal,
    RandomShaped,
    Lax,
    Custom,
}

#[derive(Clone)]
pub struct Hotel {
    pub id: String,
    num_rooms: usize,
    capital: f64,
    building_type: BuildingType,
    elevator_position: usize,
    rooms_per_story: usize,
    entry_fee: f64,
    daily_costs: f64,
    pub apartments: Vec<Apartment>,
}

impl Hotel {
    pub fn new(id: String, num_rooms: usize, capital: f64, building_type: BuildingType, elevator_position: usize, rooms_per_story: usize, entry_fee: f64, daily_costs: f64) -> Self {
        Self {
            id,
            num_rooms,
            capital,
            building_type,
            elevator_position,
            rooms_per_story,
            entry_fee,
            daily_costs,
            apartments: Vec::new(),
        }
    }

    pub fn add_apartment(&mut self, apartment: Apartment) {
        self.apartments.push(apartment);
    }

    pub fn get_room(&self, apartment_number: usize) -> Option<(usize, usize)> {
        self.apartments.iter().find(|&a| a.number == apartment_number).map(|a| (a.number, a.floor))
    }

    pub fn available_rooms(&self) -> usize {
        self.apartments.iter().filter(|a| a.resident.is_none()).count()
    }

    pub fn find_next_available_room(&self) -> Option<usize> {
        self.apartments.iter().position(|apt| apt.resident.is_none())
    }

    pub fn add_resident(&mut self, resident: Resident, apartment_number: usize) {
        if let Some(apartment) = self.apartments.get_mut(apartment_number) {
            if apartment.resident.is_none() {
                apartment.resident = Some(Arc::new(resident));
            } else {
                println!("Apartment is already occupied.");
            }
        } else {
            println!("Invalid apartment number.");
        }
    }
}
