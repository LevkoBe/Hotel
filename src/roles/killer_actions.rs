use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
pub enum KillerAction {
    Kill,
    Rob,
    Bribe,
    Threaten,
}
