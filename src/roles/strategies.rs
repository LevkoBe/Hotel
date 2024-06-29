use crate::hotel;

use super::roles::Role;

pub trait ResidentStrategy: Send + Sync {
    fn perform_action(&self, hotel: &mut hotel::Hotel);
    fn confess_role(&self) -> Role;
}

pub struct KillerStrategy;
impl ResidentStrategy for KillerStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Killer is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Killer
    }
}

pub struct PolicemanStrategy;
impl ResidentStrategy for PolicemanStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Policeman is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Policeman
    }
}

pub struct DoctorStrategy;
impl ResidentStrategy for DoctorStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Doctor is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Doctor
    }
}

pub struct JanitorStrategy;
impl ResidentStrategy for JanitorStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Janitor is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Janitor
    }
}

pub struct OldWomanStrategy;
impl ResidentStrategy for OldWomanStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("OldWoman is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::OldWoman
    }
}

pub struct SwindlerStrategy;
impl ResidentStrategy for SwindlerStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Swindler is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Swindler
    }
}

pub struct AvengerStrategy;
impl ResidentStrategy for AvengerStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Avenger is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Avenger
    }
}

pub struct JudgeStrategy;
impl ResidentStrategy for JudgeStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Judge is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Judge
    }
}

pub struct ProfessorStrategy;
impl ResidentStrategy for ProfessorStrategy {
    fn perform_action(&self, hotel: &mut hotel::Hotel) {
        println!("Professor is performing action");
    }
    fn confess_role(&self) -> Role {
        Role::Professor
    }
}
