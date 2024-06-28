use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy)]
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
