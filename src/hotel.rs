use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{Arc, Mutex};
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
            apartments: Hotel::initialize_apartments(num_rooms),
            available_roles,
        }
    }

    pub fn initialize_apartments(num_rooms: usize) -> Vec<Apartment> {
        let mut apartments = Vec::new();
        for i in 0..num_rooms {
            apartments.push(Apartment::new(i, 0));
        }
        apartments
    }

    pub fn get_all_residents(&self) -> Vec<Arc<Mutex<Resident>>> {
        let mut residents = Vec::new();
        for apt in &self.apartments {
            if let Some(resident) = &apt.resident {
                residents.push(Arc::clone(resident));
            }
        }
        residents
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

    pub fn available_rooms(&self) -> Vec<usize> {
        self.apartments
            .iter()
            .enumerate()
            .filter_map(|(index, apt)| {
                if apt.is_available() {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn available_rooms_count(&self) -> usize {
        self.apartments.iter().filter(|a| a.is_available()).count()
    }

    pub fn find_next_available_room(&self) -> Option<usize> {
        self.apartments.iter().position(|apt| apt.is_available())
    }

    pub fn is_room_available(&self, room_number: usize) -> bool {
        self.apartments
            .get(room_number)
            .map(|a| a.is_available())
            .unwrap_or(false)
    }

    pub fn add_resident(&mut self, resident: Resident, apartment_number: usize) {
        if let Some(apartment) = self.apartments.get_mut(apartment_number) {
            apartment.assign_resident(resident);
        } else {
            println!("Invalid apartment number.");
        }
    }
}
