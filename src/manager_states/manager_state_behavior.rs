use super::handling_result::HandlingResult;
use crate::{game_flow, hotel};

pub trait ManagerStateBehavior {
    fn handle_command(
        &mut self,
        game_flow: &mut Option<game_flow::GameFlow>,
        input: &[&str],
    ) -> HandlingResult;
    fn finish_setting(&self) -> hotel::Hotel;
}
