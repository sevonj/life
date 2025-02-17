// SPDX-License-Identifier: LGPL-3.0-or-later
//! class: [WorldEnv]
//! desc: Manages environment stuff, like the sun and the skybox.
//!
use std::f32::consts::PI;
use std::time::Duration;

use godot::classes::DirectionalLight3D;
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(base=Node)]
pub struct WorldEnv {
    sun: Gd<DirectionalLight3D>,

    base: Base<Node>,
}

#[godot_api]
impl INode for WorldEnv {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            sun: DirectionalLight3D::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_env();

        // Vaguely just after midday-ish
        self.sun
            .set_rotation_degrees(Vector3::new(-56.8, -24.1, 28.2));
    }
}

impl WorldEnv {
    fn setup_env(&mut self) {
        let mut sun = self.sun.clone();
        sun.set_name("env_sun");

        self.base_mut().add_child(&sun);
    }

    pub fn set_time(&mut self, time_of_day: Duration) {
        let angle = time_of_day.as_secs_f32() / (3600.0 * 24.0) * PI * 2.0 + PI;
        self.sun.set_rotation(Vector3::new(angle, -24.0, 0.0));
    }
}
