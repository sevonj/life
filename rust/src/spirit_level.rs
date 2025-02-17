// SPDX-License-Identifier: LGPL-3.0-or-later
use godot::prelude::*;

use godot::classes::{CsgCylinder3D, CsgSphere3D, Material};

use crate::Person;

const Y_OFFSET: f32 = 2.2;
const VIAL_R: f32 = 0.06;
const VIAL_H: f32 = 0.4;
const VIAL_RO: f32 = VIAL_R + 0.01;

#[derive(Debug, GodotClass)]
#[class(base=Node3D)]
pub struct SpiritLevel {
    pub target: Option<Gd<Person>>,

    vial: Gd<CsgCylinder3D>,
    vial_contents: Gd<CsgCylinder3D>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for SpiritLevel {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            target: None,
            vial: CsgCylinder3D::new_alloc(),
            vial_contents: CsgCylinder3D::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        self.setup_visuals();
    }

    fn process(&mut self, _delta: f64) {
        if let Some(target) = self.target.clone() {
            self.base_mut().show();

            let position = target.get_global_position() + Vector3::UP * Y_OFFSET;
            let mood = target.bind().needs().sleep() as f32;

            self.base_mut().set_global_position(position);

            self.vial_contents
                .set_position(Vector3::DOWN * ((1.0 - mood) * VIAL_H / 2.0));
            self.vial_contents.set_height((1.0 - mood) * VIAL_H);
        } else {
            self.base_mut().hide();
        }
    }
}

impl SpiritLevel {
    fn setup_visuals(&mut self) {
        let mut vial_topball = CsgSphere3D::new_alloc();
        vial_topball.set_radius(VIAL_RO);
        vial_topball.set_position(Vector3::UP * (VIAL_H / 2.0 - VIAL_R));

        let mut vial_bottomball = CsgSphere3D::new_alloc();
        vial_bottomball.set_radius(VIAL_RO);
        vial_bottomball.set_position(Vector3::DOWN * (VIAL_H / 2.0 - VIAL_R));

        let vial_mat: Gd<Material> = load("res://assets/materials/fx/mat_fx_spiritlevel_vial.tres");

        let mut vial = self.vial.clone();
        vial.set_height(VIAL_H - VIAL_RO * 2.0);
        vial.set_radius(VIAL_R);
        vial.add_child(&vial_topball);
        vial.add_child(&vial_bottomball);
        vial.set_material_override(&vial_mat);
        vial.set_name("vial");

        let mut contents_topball = CsgSphere3D::new_alloc();
        contents_topball.set_radius(VIAL_R);
        contents_topball.set_position(Vector3::UP * (VIAL_H / 2.0 - VIAL_R));

        let mut contents_bottomball = CsgSphere3D::new_alloc();
        contents_bottomball.set_radius(VIAL_R);
        contents_bottomball.set_position(Vector3::DOWN * (VIAL_H / 2.0 - VIAL_R));

        let contents_mat: Gd<Material> =
            load("res://assets/materials/fx/mat_fx_spiritlevel_vial_contents.tres");

        let mut contents = CsgCylinder3D::new_alloc();
        contents.set_height(VIAL_H - VIAL_R * 2.0);
        contents.set_radius(VIAL_R);
        contents.add_child(&contents_topball);
        contents.add_child(&contents_bottomball);

        contents.set_material_override(&contents_mat);
        contents.set_name("contents");

        self.base_mut().add_child(&vial);
        self.base_mut().add_child(&contents);
    }
}
