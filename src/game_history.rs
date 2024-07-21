use crate::{hotel, resident::Resident};
use std::sync::MutexGuard;

#[derive(Debug)]
pub struct Action {
    pub day: usize,
    pub actor: usize,
    pub action_type: String,
    pub target: usize,
}

pub struct GameHistory {
    pub actions: Vec<Action>,
    pub day: usize,
}

impl GameHistory {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            day: 1,
        }
    }

    pub fn add_action(
        &mut self,
        actor: usize,
        action_type: String,
        target: usize,
        day: Option<usize>,
    ) {
        let day = day.unwrap_or(self.day);
        self.actions.push(Action {
            day,
            actor,
            action_type,
            target,
        });
    }

    pub fn retell_all_history(&self, hotel: &hotel::Hotel, format: Option<&str>) -> String {
        self.retell_history(hotel, format, None)
    }

    pub fn retell_last_night(&self, hotel: &hotel::Hotel, format: Option<&str>) -> String {
        if self.actions.is_empty() {
            return "Nothing happened?!?".to_string();
        }
        self.retell_history(hotel, format, Some(self.day))
    }

    fn retell_history(
        &self,
        hotel: &hotel::Hotel,
        format: Option<&str>,
        day_filter: Option<usize>,
    ) -> String {
        let mut output = String::new();
        for action in &self.actions {
            if let Some(day) = day_filter {
                if action.day != day {
                    continue;
                }
            }

            let actor = hotel.apartments[action.actor]
                .resident
                .as_ref()
                .unwrap()
                .lock()
                .unwrap();

            let actor_format = format.unwrap_or("n (r)");

            let actor_info: String = actor_format
                .chars()
                .map(|c| self.format_resident_detail(&actor, c))
                .collect();

            let target_info = if let Some(target) = &hotel.apartments[action.target].resident {
                let target = &target.lock().unwrap();
                actor_format
                    .chars()
                    .map(|c| self.format_resident_detail(target, c))
                    .collect()
            } else {
                "None".to_string()
            };

            output.push_str(&format!(
                "On day {}, {} {} {}\n",
                action.day, actor_info, action.action_type, target_info
            ));
        }
        output
    }

    fn format_resident_detail(&self, resident: &MutexGuard<Resident>, param: char) -> String {
        match param {
            '#' => format!("{}", resident.apartment_number),
            '$' => format!("{:.2}", resident.account_balance),
            'a' => format!("{}", resident.age),
            'n' => format!("{}", resident.name),
            's' => format!("{:?}", resident.status),
            'r' => format!("{}", resident.strategy.confess_role()),
            't' => format!("{:?}", resident.resident_type),
            _ => param.to_string(),
        }
    }

    pub fn next_day(&mut self) {
        self.day += 1;
    }

    pub fn has_visited(&self, actor: usize, target_apartment: usize) -> bool {
        self.actions
            .iter()
            .any(|action| action.actor == actor && action.target == target_apartment)
    }
}
