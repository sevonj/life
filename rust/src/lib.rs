mod action;
mod action_advertisement;
mod camera_rig_orbit;
mod entity_collider;
pub mod furniture;
mod person;
mod person_ai;
mod person_needs;
mod ui_home_taskbar;
mod ui_person_bio_panel;
mod ui_person_needs_panel;
mod world;
mod world_env;

pub use action::Action;
pub use action_advertisement::{ActionAdvertisement, ActionAdvertisementStat};
pub use camera_rig_orbit::CameraRigOrbit;
pub use entity_collider::EntityCollider;
pub use person::Person;
pub use person_ai::PersonAi;
pub use person_needs::PersonNeeds;
pub use ui_home_taskbar::UiHomeTaskbar;
pub use ui_person_bio_panel::UiPersonBioPanel;
pub use ui_person_needs_panel::UiPersonNeedsPanel;
pub use world::World;
pub use world_env::WorldEnv;

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
