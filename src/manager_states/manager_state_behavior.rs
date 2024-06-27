pub trait ManagerStateBehavior {
    fn handle_command(&mut self, manager: &mut crate::manager::Manager, input: &[&str]);
}
