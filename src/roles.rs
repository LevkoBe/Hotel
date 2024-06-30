use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Newcomer,
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
