use super::handling_result::HandlingResult;
use crate::hotel;

pub trait ManagerStateBehavior {
    fn handle_command(
        &mut self,
        hotel: &mut Option<hotel::Hotel>,
        input: &[&str],
    ) -> HandlingResult;
    fn finish_setting(&self, hotel: Option<hotel::Hotel>) -> hotel::Hotel;
}
