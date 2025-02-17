// SPDX-License-Identifier: LGPL-3.0-or-later
use godot::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum WorldViewMode {
    Build,
    Buy,
    #[default]
    Play,
}
