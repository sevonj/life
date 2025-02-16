use godot::classes::{CsgSphere3D, ICsgSphere3D, Material};

use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToolGizmoStyle {
    #[default]
    Normal,
    Destructive,
}

#[derive(Debug, GodotClass)]
#[class(base=CsgSphere3D)]
pub struct ToolGizmo {
    style: ToolGizmoStyle,

    base: Base<CsgSphere3D>,
}

#[godot_api]
impl ICsgSphere3D for ToolGizmo {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            style: ToolGizmoStyle::default(),

            base,
        }
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

    pub fn style(&self) -> ToolGizmoStyle {
        self.style
    }

    pub fn set_style(&mut self, style: ToolGizmoStyle) {
        let material: Gd<Material> = match style {
            ToolGizmoStyle::Normal => load("res://assets/materials/fx/mat_fx_uvcube.tres"),
            ToolGizmoStyle::Destructive => load("res://assets/materials/fx/mat_fx_uvcube_red.tres"),
        };
        self.base_mut().set_material(&material);
        self.style = style;
    }
}
