use core::fmt;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Killer,
    Police,
    Doctor,
    Janitor,
    OldWoman,
    Swindler,
    Avenger,
    Judge,
    Professor,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            Role::Killer => "Killer",
            Role::Police => "Police",
            Role::Doctor => "Doctor",
            Role::Janitor => "Janitor",
            Role::OldWoman => "OldWoman",
            Role::Swindler => "Swindler",
            Role::Avenger => "Avenger",
            Role::Judge => "Judge",
            Role::Professor => "Professor",
        };
        write!(f, "{}", role_str)
    }
}
