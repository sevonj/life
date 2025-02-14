use godot::classes::{CsgSphere3D, ICsgSphere3D, Material};

use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(base=CsgSphere3D)]

pub struct ToolGizmo {
    base: Base<CsgSphere3D>,
}

#[godot_api]
impl ICsgSphere3D for ToolGizmo {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        self.setup_model();
    }
}

impl ToolGizmo {
    fn setup_model(&mut self) {
        self.base_mut().set_radius(0.2);
        self.base_mut().set_smooth_faces(false);
        self.base_mut().set_radial_segments(4);
        self.base_mut().set_rings(2);
        let material: Gd<Material> = load("res://assets/materials/fx/mat_fx_uvcube.tres");
        self.base_mut().set_material(&material);
    }
}
