use super::handling_result::HandlingResult;
use crate::game_flow;

pub trait ManagerStateBehavior {
    fn handle_command(
        &mut self,
        game_flow: &mut game_flow::GameFlow,
        input: &[&str],
    ) -> HandlingResult;
}
