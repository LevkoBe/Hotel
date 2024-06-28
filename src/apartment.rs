use crate::resident::Resident;
use std::sync::Arc;

#[derive(Clone)]
pub struct Apartment {
    pub number: usize,
    pub floor: usize,
    pub resident: Option<Arc<Resident>>,
}

impl Apartment {
    pub fn new(number: usize, floor: usize) -> Apartment {
        Apartment {
            number,
            floor,
            resident: None,
        }
    }

    pub fn assign_resident(&mut self, resident: Arc<Resident>) {
        self.resident = Some(resident);
    }

    pub fn is_available(&self) -> bool {
        self.resident.is_none()
    }
}
