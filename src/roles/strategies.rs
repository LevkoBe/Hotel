pub trait ResidentStrategy: Send + Sync {
    fn perform_action(&self);
}

pub struct KillerStrategy;
impl ResidentStrategy for KillerStrategy {
    fn perform_action(&self) {
        println!("Killer is performing action");
    }
}

pub struct PolicemanStrategy;
impl ResidentStrategy for PolicemanStrategy {
    fn perform_action(&self) {
        println!("Policeman is performing action");
    }
}

pub struct DoctorStrategy;
impl ResidentStrategy for DoctorStrategy {
    fn perform_action(&self) {
        println!("Doctor is performing action");
    }
}

pub struct JanitorStrategy;
impl ResidentStrategy for JanitorStrategy {
    fn perform_action(&self) {
        println!("Janitor is performing action");
    }
}

pub struct OldWomanStrategy;
impl ResidentStrategy for OldWomanStrategy {
    fn perform_action(&self) {
        println!("OldWoman is performing action");
    }
}

pub struct SwindlerStrategy;
impl ResidentStrategy for SwindlerStrategy {
    fn perform_action(&self) {
        println!("Swindler is performing action");
    }
}

pub struct AvengerStrategy;
impl ResidentStrategy for AvengerStrategy {
    fn perform_action(&self) {
        println!("Avenger is performing action");
    }
}

pub struct JudgeStrategy;
impl ResidentStrategy for JudgeStrategy {
    fn perform_action(&self) {
        println!("Judge is performing action");
    }
}

pub struct ProfessorStrategy;
impl ResidentStrategy for ProfessorStrategy {
    fn perform_action(&self) {
        println!("Professor is performing action");
    }
}
