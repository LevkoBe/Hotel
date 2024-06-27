use crate::resident::Resident;
use std::sync::Arc;

pub struct Apartment {
    number: usize,
    floor: usize,
    resident: Option<Arc<Resident>>,
}

impl Apartment {
    pub fn new(number: usize, floor: usize) -> Self {
        Self {
            number,
            floor,
            resident: None,
        }
    }

    pub fn assign_resident(&mut self, resident: Arc<Resident>) {
        self.resident = Some(resident);
    }
}
