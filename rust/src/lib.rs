mod action;
mod action_advertisement;
mod camera_rig_orbit;
mod entity_collider;
mod furniture;
mod lot_builder;
mod lot_builder_grid;
mod lot_walls;
mod person;
mod person_ai;
mod person_needs;
mod ui_person_bio_panel;
mod ui_person_needs_panel;
mod ui_world_mode_select;
mod ui_world_taskbar;
mod world;
mod world_env;
mod world_view_mode;

pub use action::Action;
pub use action_advertisement::{ActionAdvertisement, ActionAdvertisementStat};
pub use camera_rig_orbit::CameraRigOrbit;
pub use entity_collider::EntityCollider;
pub use furniture::Furniture;
pub use lot_builder::LotBuilder;
pub use lot_builder_grid::LotBuilderGrid;
pub use lot_walls::LotWalls;
pub use person::{Person, Task};
pub use person_ai::PersonAi;
pub use person_needs::PersonNeeds;
pub use ui_person_bio_panel::UiPersonBioPanel;
pub use ui_person_needs_panel::UiPersonNeedsPanel;
pub use ui_world_mode_select::UiWorldModeSelectOld;
pub use ui_world_taskbar::UiWorldTaskbar;
pub use world::World;
pub use world_env::WorldEnv;
pub use world_view_mode::WorldViewMode;

use godot::prelude::*;

struct HotReload;

#[gdextension]
unsafe impl ExtensionLibrary for HotReload {
    fn on_level_init(_level: InitLevel) {
        println!("[Rust]      Init level {:?}", _level);
    }

    fn on_level_deinit(_level: InitLevel) {
        println!("[Rust]      Deinit level {:?}", _level);
    }
}
