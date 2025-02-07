use godot::{classes::Node3D, obj::Gd};

use crate::furniture::Furniture;

/// Describes an action a Person can perform on an entity.
#[derive(Debug, Clone)]
pub struct ActionAdvertisement {
    /// The object responsible for this advertisement
    pub source_node: Gd<Furniture>,
    /// String key of the action
    pub action_key: String,
    /// What kind of needs does this action appel
    pub stats: Vec<ActionAdvertisementStat>,
}

#[derive(Debug, Clone)]
pub struct ActionAdvertisementStat {
    /// String key of the need this advertisement promises to satisfy.
    pub key: String,
    /// Higher means more desireable. Zero means no effect. Can be negative for discouragement.
    pub value: isize,
}
