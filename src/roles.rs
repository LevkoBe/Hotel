pub trait RoleStrategy {
    fn perform_action(&self, resident: &crate::resident::Resident);
}

pub struct KillerStrategy;
impl RoleStrategy for KillerStrategy {
    fn perform_action(&self, resident: &crate::resident::Resident) {
        // Killer action logic
    }
}

pub struct PolicemanStrategy;
impl RoleStrategy for PolicemanStrategy {
    fn perform_action(&self, resident: &crate::resident::Resident) {
        // Policeman action logic
    }
}

// Add other role strategies as needed
