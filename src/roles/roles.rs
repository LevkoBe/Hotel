use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Newcomer,
    Killer,
    Policeman,
    Doctor,
    Janitor,
    OldWoman,
    Swindler,
    Avenger,
    Judge,
    Professor,
}
