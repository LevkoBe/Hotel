use std::sync::Arc;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

use crate::{apartment::Apartment, resident::Resident, roles::roles::Role};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum BuildingType {
    Rectangular,
    Pyramidal,
    RandomShaped,
    Lax,
    Custom,
}

#[allow(dead_code)]
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
    pub available_roles: Vec<Role>,
}

impl Hotel {
    pub fn new(
        id: String,
        num_rooms: usize,
        capital: f64,
        building_type: BuildingType,
        elevator_position: usize,
        rooms_per_story: usize,
        entry_fee: f64,
        daily_costs: f64,
    ) -> Self {
        let possible_roles: Vec<Role> = Role::iter().collect();
        let roles_count = possible_roles.len();
        let mut available_roles = Vec::new();
        for i in 0..num_rooms {
            available_roles.push(possible_roles[i % roles_count].clone());
        }
        let mut rng = thread_rng();
        available_roles.shuffle(&mut rng);
        
        Self {
            id,
            num_rooms,
            capital,
            building_type,
            elevator_position,
            rooms_per_story,
            entry_fee,
            daily_costs,
            apartments: vec![Apartment::new(0, 0); num_rooms],
            available_roles,
        }
    }

    pub fn get_room(&self, apartment_number: usize) -> Option<(usize, usize)> {
        self.apartments
            .iter()
            .find(|&a| a.number == apartment_number)
            .map(|a| (a.number, a.floor))
    }

    pub fn random_available_role(&mut self) -> Option<Role> {
        self.available_roles.pop()
    }

    pub fn available_rooms(&self) -> usize {
        self.apartments
            .iter()
            .filter(|a| a.resident.is_none())
            .count()
    }

    pub fn find_next_available_room(&self) -> Option<usize> {
        self.apartments
            .iter()
            .position(|apt| apt.resident.is_none())
    }

    pub fn is_room_available(&self, room_number: usize) -> bool {
        self.apartments
            .get(room_number)
            .map(|a| a.resident.is_none())
            .unwrap_or(false)
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
