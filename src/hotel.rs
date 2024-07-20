use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

use crate::mail::Suspicion;
use crate::text_formatters::format_to_length;
use crate::{apartment::Apartment, resident::Resident, roles::Role};

const FORMAT_LENGTH_LEFT: usize = 2;
const FORMAT_LENGTH_RIGHT: usize = 6;
const APARTMENT_WIDTH: usize = 10;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum BuildingType {
    Rectangular,
    Pyramidal,
    RandomShaped,
    Lax,
    Custom,
}

#[derive(Serialize, Deserialize)]
pub struct Hotel {
    pub id: String,
    pub num_rooms: usize,
    pub capital: f64,
    pub building_type: BuildingType,
    pub elevator_position: usize,
    pub rooms_per_story: usize,
    pub entrance_fee: f64,
    pub daily_costs: f64,
    #[serde(skip)]
    pub apartments: Vec<Apartment>,
    #[serde(skip)]
    pub available_roles: Vec<Role>,
    #[serde(skip)]
    pub announcements: Vec<String>,
    #[serde(skip)]
    pub police_suspicions: Vec<Suspicion>,
    #[serde(skip)]
    pub investigation_queue: HashMap<usize, Suspicion>,
    #[serde(skip)]
    pub credible_sources: Vec<usize>,
}

impl Hotel {
    pub fn new(
        id: String,
        num_rooms: usize,
        capital: f64,
        building_type: BuildingType,
        elevator_position: usize,
        rooms_per_story: usize,
        entrance_fee: f64,
        daily_costs: f64,
    ) -> Self {
        Self {
            id,
            num_rooms,
            capital,
            building_type,
            elevator_position,
            rooms_per_story,
            entrance_fee,
            daily_costs,
            apartments: vec![],
            available_roles: vec![],
            announcements: vec![],
            police_suspicions: vec![],
            investigation_queue: HashMap::new(),
            credible_sources: vec![],
        }
    }

    pub fn get_ready_apartments(&self, own_apartment: Option<usize>) -> Vec<usize> {
        self.apartments
            .iter()
            .filter_map(|apartment| {
                if apartment.is_opened {
                    if let Some(own_apartment) = own_apartment {
                        if apartment.number != own_apartment {
                            Some(apartment.number)
                        } else {
                            None
                        }
                    } else {
                        Some(apartment.number)
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn reinitialize(&mut self) {
        let possible_roles: Vec<Role> = Role::iter().collect();
        let roles_count = possible_roles.len();
        let mut available_roles = Vec::new();
        for i in 0..self.num_rooms {
            available_roles.push(possible_roles[i % roles_count].clone());
        }
        let mut rng = thread_rng();
        available_roles.shuffle(&mut rng);

        self.apartments = Hotel::initialize_apartments(self.num_rooms, self.rooms_per_story);
        self.available_roles = available_roles;
    }

    pub fn print_hotel(&self, style: &str, destination: Option<usize>, player: Option<&Resident>) {
        let re = Regex::new(r"^.{4}$").unwrap(); // any style
                                                 // let re = Regex::new(r"^[#\$atsrnp]{4}$").unwrap();   // some specific info

        if re.is_match(style) {
            self.print_detailed(style);
        } else {
            match style {
                "default" => self.print_detailed("#nsr"),
                "move" => {
                    if let (Some(dest), Some(player)) = (destination, player) {
                        self.print_move(dest, player);
                    } else {
                        println!("Destination and player are required for 'move' style");
                    }
                }
                _ => println!("Invalid style"),
            }
        }
    }

    pub fn announce(&mut self) {
        let mut announcement = String::new();
        println!("Please, announce:");
        io::stdin().read_line(&mut announcement).unwrap();
        self.announcements.push(announcement);
    }

    pub fn send_mail(&mut self, apartment: usize, mail: String) {
        self.apartments[apartment].mails.push(mail);
    }

    fn print_detailed(&self, custom_params: &str) {
        let mut output = String::new();

        let total_floors = (self.num_rooms as f64 / self.rooms_per_story as f64).ceil() as usize;

        for floor in (0..total_floors).rev() {
            let mut line0 = String::new();
            let mut line1 = String::new();
            let mut line2 = String::new();

            for room in 0..self.rooms_per_story {
                let idx = floor * self.rooms_per_story + room;

                if room == self.elevator_position {
                    line0.push_str("|^v|");
                    line1.push_str("|^v|");
                    line2.push_str("|^v|");
                }
                if idx >= self.apartments.len() {
                    line0.push_str(&format!("|{:=^width$}|", "", width = APARTMENT_WIDTH));
                    line1.push_str(&format!("|{: ^width$}|", "", width = APARTMENT_WIDTH));
                    line2.push_str(&format!(
                        "|{: ^width$}🚪 |",
                        "",
                        width = APARTMENT_WIDTH - 3
                    ));
                } else {
                    let details = custom_params
                        .chars()
                        .map(|param| self.format_apartment_detail(&self.apartments[idx], param))
                        .collect::<Vec<String>>();

                    line0.push_str(&format!("|{:=^width$}|", "", width = APARTMENT_WIDTH));
                    line1.push_str(&format!(
                        "|{}: {}|",
                        format_to_length(&details[0], FORMAT_LENGTH_LEFT),
                        format_to_length(&details[1], FORMAT_LENGTH_RIGHT)
                    ));
                    line2.push_str(&format!(
                        "|{}: {}|",
                        format_to_length(&details[2], FORMAT_LENGTH_LEFT),
                        format_to_length(&details[3], FORMAT_LENGTH_RIGHT)
                    ));
                }
            }

            output.push_str(&line0);
            output.push('\n');
            output.push_str(&line1);
            output.push('\n');
            output.push_str(&line2);
            output.push('\n');
        }

        println!("{}", output);
    }

    fn print_move(&self, destination: usize, player: &Resident) {
        let mut output = String::new();

        let total_floors = (self.num_rooms as f64 / self.rooms_per_story as f64).ceil() as usize;

        for floor in (0..total_floors).rev() {
            let mut line = String::new();

            for room in 0..self.rooms_per_story {
                let idx = floor * self.rooms_per_story + room;

                if room == self.elevator_position {
                    line.push_str("| ^v |");
                }
                if idx >= self.apartments.len() {
                    line.push_str("| 🚪 |");
                } else {
                    let symbol = if idx == destination {
                        '+'
                    } else if idx == player.current_position {
                        'x'
                    } else {
                        'E'
                    };
                    line.push_str(&format!("|{:02} {}|", idx, symbol));
                }
            }

            output.push_str(&line);
            output.push('\n');
        }

        println!("{}", output);
    }

    fn format_apartment_detail(&self, apartment: &Apartment, param: char) -> String {
        if let Some(resident) = &apartment.resident {
            let resident = resident.lock().unwrap();
            match param {
                '#' => format!("{}", apartment.number),
                '$' => format!("{:.2}", resident.account_balance),
                'a' => format!("{}", resident.age),
                'n' => format!("{}", resident.name),
                's' => format!("{:?}", resident.status),
                'r' => format!("{}", resident.strategy.confess_role()),
                't' => format!("{:?}", resident.resident_type),
                'p' => format!("{}", resident.current_position),
                _ => format!("{} {:-^width$}", param, "", width = APARTMENT_WIDTH),
            }
        } else {
            "Vacant".to_string()
        }
    }

    pub fn initialize_apartments(num_rooms: usize, rooms_per_story: usize) -> Vec<Apartment> {
        let mut apartments = Vec::new();
        for i in 0..num_rooms {
            let floor = i / rooms_per_story;
            apartments.push(Apartment::new(i, floor));
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
            .map(|a| (a.number % self.rooms_per_story, a.floor))
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

    pub fn save(&self) -> io::Result<()> {
        let path = format!("hotel_configs/{}.json", self.id);
        let mut file = File::create(path)?;
        let hotel_data = serde_json::to_string(self)?;
        write!(file, "{}", hotel_data)?;
        Ok(())
    }

    pub fn upload(id: &str) -> Option<Self> {
        let path = format!("hotel_configs/{}.json", id);
        if !Path::new(&path).exists() {
            return None;
        }

        let mut file = File::open(path).ok()?;
        let mut hotel_data = String::new();
        file.read_to_string(&mut hotel_data).ok()?;
        let hotel: Hotel = serde_json::from_str(&hotel_data).ok()?;
        Some(hotel)
    }
}
