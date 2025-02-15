use godot::prelude::*;
use uuid::Uuid;

use crate::{Furniture, Person};

#[derive(Debug, Clone)]
pub struct Action {
    pub key: String,
    pub object: Option<Gd<Furniture>>,
    /// Pair activity partner
    pub partner_uuid: Option<Uuid>,
    /// (Pair activity), is this action the master, or a secondary copy.
    pub primary: bool,
}

impl Action {
    pub fn idle() -> Self {
        Self {
            key: "idle".into(),
            object: None,
            partner_uuid: None,
            primary: true,
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
