//! class: [WorldEnv]
//! desc: Manages environment stuff, like the sun and the skybox.
//!
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
}
