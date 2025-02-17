// SPDX-License-Identifier: LGPL-3.0-or-later
use godot::prelude::*;
use uuid::Uuid;

use crate::Furniture;

#[derive(Debug, Clone)]
pub struct Action {
    pub key: String,
    pub object: Option<Gd<Furniture>>,
    /// Pair activity partner
    pub partner_uuid: Option<Uuid>,
    /// (Pair activity), Master copy of the task is
    pub master_uuid: Option<Uuid>,
}

impl Action {
    pub fn idle() -> Self {
        Self {
            key: "idle".into(),
            object: None,
            partner_uuid: None,
            master_uuid: None,
        }
    }

    pub fn is_primary(&self) -> bool {
        self.master_uuid.is_none()
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
