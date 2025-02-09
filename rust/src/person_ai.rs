//! class: [PersonAi]
//! desc: AI for [crate::Person]
//!
use std::collections::HashSet;

use rand;

use crate::{Action, ActionAdvertisement, PersonNeeds};

struct ActionTemp {
    action: Action,
    score: f64,
}

#[derive(Debug)]
pub struct PersonAi {
    pub last_action: String,
}

impl Default for PersonAi {
    fn default() -> Self {
        Self {
            last_action: "".into(),
        }
    }
}

impl PersonAi {
    pub fn decide_action(
        &self,
        needs: &PersonNeeds,
        advertised_actions: &Vec<ActionAdvertisement>,
        possible_actions: &Vec<String>,
    ) -> Action {
        let mut processed_actions = vec![];

        for advert in advertised_actions {
            if !possible_actions.contains(&advert.action_key)
                || advert.source_node.bind().is_reserved()
            {
                continue;
            }

            let action = Action {
                key: advert.action_key.clone(),
                target: Some(advert.source_node.clone()),
            };

            let mut score = self.score_action_by_needs(needs, advert);
            score += self.score_action_by_history(advert);

            processed_actions.push(ActionTemp { action, score });
        }
        if processed_actions.is_empty() {
            return Action::idle();
        }

        processed_actions.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        filter_action_dupes(&mut processed_actions);

        let choice = processed_actions.len()
            - 1
            - match processed_actions.len() {
                1 => 0,
                2 => rand::random_range(0..=1),
                _ => rand::random_range(0..=2),
            };

        processed_actions[choice].action.clone()
    }

    fn score_action_by_needs(&self, needs: &PersonNeeds, advert: &ActionAdvertisement) -> f64 {
        let mut score = 0.0;

        for stat in &advert.stats {
            let value = stat.value as f64;
            match stat.key.as_str() {
                "bladder" => score += value / needs.bladder(),
                "comfort" => score += value / needs.comfort(),
                "environment" => score += value / needs.environment(),
                "fun" => score += value / needs.fun(),
                "hunger" => score += value / needs.hunger(),
                "hygiene" => score += value / needs.hygiene(),
                "sleep" => score += value / needs.sleep(),
                "social" => score += value / needs.social(),
                _ => (),
            }
        }

        score
    }

    fn score_action_by_history(&self, advert: &ActionAdvertisement) -> f64 {
        if advert.action_key == self.last_action {
            return -100.0;
        }
        0.0
    }
}

/// Only keep the highest ranking action of each type
fn filter_action_dupes(actions: &mut Vec<ActionTemp>) {
    let mut dupe_check = HashSet::new();
    for i in (0..actions.len()).rev() {
        let action = &actions[i];
        let key = action.action.key.clone();
        if dupe_check.contains(&key) {
            actions.remove(i);
        }
        dupe_check.insert(key);
    }
}
