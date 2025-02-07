use godot::prelude::*;

use crate::furniture::Furniture;

#[derive(Debug, Clone)]
pub struct Action {
    pub key: String,
    pub target: Option<Gd<Furniture>>,
}

impl Action {
    pub fn idle() -> Self {
        Self {
            key: "idle".into(),
            target: None,
        }
    }

    pub fn to_present_tense(&self) -> String {
        match self.key.as_str() {
            "make_food" => "Making food".into(),
            "toilet" => "Toiletting".into(),
            "sit" => "Sitting".into(),
            "sleep" => "Sleeping".into(),
            "do_the_mario" => "Doing the Mario".into(),
            "idle" => "Idling".into(),
            _ => format!("Unknown activity: `{}`", self.key),
        }
    }
}
