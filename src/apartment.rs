use crate::resident::Resident;
use std::sync::{Arc, Mutex};

pub struct Apartment {
    pub is_opened: bool,
    pub number: usize,
    pub floor: usize,
    pub guests: Vec<usize>,
    pub resident: Option<Arc<Mutex<Resident>>>,
    pub mails: Vec<String>,
}

impl Apartment {
    pub fn new(number: usize, floor: usize) -> Apartment {
        Apartment {
            is_opened: true,
            number,
            floor,
            guests: vec![],
            resident: None,
            mails: vec![],
        }
    }

    pub fn assign_resident(&mut self, resident: Resident) {
        if self.is_available() {
            self.resident = Some(Arc::new(Mutex::new(resident)));
        } else {
            println!("Apartment is already occupied.");
        }
    }

    pub fn read_mails(&self) {
        for i in 0..self.mails.len() {
            println!("Mail {}: {}", i + 1, self.mails[i]);
        }
    }

    pub fn clear_mails(&mut self) {
        self.mails.clear();
    }

    pub fn is_available(&self) -> bool {
        self.resident.is_none()
    }
}
