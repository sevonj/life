//! class: [PersonAi]
//! desc: AI for [crate::Person]
//!
use std::collections::HashSet;

use godot::global::godot_print;
use rand;

use crate::{
    action, furniture::Furniture, Action, ActionAdvertisement, EntityCollider, PersonNeeds,
};

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
        struct ActionTemp {
            action: Action,
            score: f64,
        }

        let mut processed_actions = vec![];
        for action in advertised_actions {
            if !possible_actions.contains(&action.action_key) {
                continue;
            }
            if action.source_node.bind().is_reserved() {
                continue;
            }
            if action.action_key == self.last_action {
                continue;
            }
            processed_actions.push(ActionTemp {
                action: Action {
                    key: action.action_key.clone(),
                    target: Some(action.source_node.clone()),
                },
                score: self.score_action(needs, action),
            });
        }
        if processed_actions.is_empty() {
            return Action::idle();
        }

        processed_actions.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        // Only keep the highest ranking action of each type
        let mut dupe_check = HashSet::new();
        for i in (0..processed_actions.len()).rev() {
            let action = &processed_actions[i];
            let key = action.action.key.clone();
            if dupe_check.contains(&key) {
                processed_actions.remove(i);
            }
            dupe_check.insert(key);
        }

        let choice = processed_actions.len()
            - 1
            - match processed_actions.len() {
                1 => 0,
                2 => rand::random_range(0..=1),
                _ => rand::random_range(0..=2),
            };
        
        processed_actions[choice].action.clone()
    }

    fn score_action(&self, needs: &PersonNeeds, action: &ActionAdvertisement) -> f64 {
        let mut score = 0.0;

        for stat in &action.stats {
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
}
