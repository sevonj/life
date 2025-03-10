// SPDX-License-Identifier: LGPL-3.0-or-later
use godot::prelude::*;

use crate::Furniture;

/// Describes an action a Person can perform on an entity.
#[derive(Debug, Clone)]
pub struct ActionAdvertisement {
    /// The object responsible for this advertisement
    pub source_node: Gd<Furniture>,
    /// String key of the action
    pub action_key: String,
    /// What kind of needs does this action appel
    pub stats: Vec<ActionAdvertisementStat>,
    /// How many people
    pub required_people: usize,
}

#[derive(Debug, Clone)]
pub struct ActionAdvertisementStat {
    /// String key of the need this advertisement promises to satisfy.
    pub key: String,
    /// Higher means more desireable. Zero means no effect. Can be negative for discouragement.
    pub value: isize,
}
