// SPDX-License-Identifier: LGPL-3.0-or-later
use godot::prelude::*;

use godot::classes::mesh::PrimitiveType;
use godot::classes::{ArrayMesh, IMeshInstance3D, Material, MeshInstance3D, SurfaceTool};


#[derive(Debug, GodotClass)]
#[class(base=MeshInstance3D)]
pub struct BuilderGrid {
    base: Base<MeshInstance3D>,
}

#[godot_api]
impl IMeshInstance3D for BuilderGrid {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let mesh = self.build_mesh();
        self.base_mut().set_mesh(&mesh);
    }
}

impl BuilderGrid {
    fn build_mesh(&self) -> Gd<ArrayMesh> {
        let mut st = SurfaceTool::new_gd();

        const LINE_W: f32 = 0.04;
        const W: f32 = LINE_W / 2.0;
        const WE: f32 = 1.0 - W;

        st.begin(PrimitiveType::TRIANGLE_STRIP);

        fn add_l(
            st: &mut SurfaceTool,
            a: Vector3,
            b: Vector3,
            c: Vector3,
            d: Vector3,
            e: Vector3,
            f: Vector3,
        ) {
            st.add_vertex(a); // Duplicate for degen triangle
            st.add_vertex(a);
            st.add_vertex(b);
            st.add_vertex(c);
            st.add_vertex(d);
            st.add_vertex(e);
            st.add_vertex(f);
            st.add_vertex(f); // Duplicate for degen triangle
        }

        fn add_tile(st: &mut SurfaceTool, offset: Vector3) {
            add_l(
                st,
                Vector3::new(0.00, 0.0, 0.25) + offset,
                Vector3::new(W, 0.0, 0.25) + offset,
                Vector3::new(0.00, 0.0, 0.00) + offset,
                Vector3::new(W, 0.0, W) + offset,
                Vector3::new(0.25, 0.0, 0.00) + offset,
                Vector3::new(0.25, 0.0, W) + offset,
            );
            add_l(
                st,
                Vector3::new(0.75, 0.0, 0.00) + offset,
                Vector3::new(0.75, 0.0, W) + offset,
                Vector3::new(1.00, 0.0, 0.00) + offset,
                Vector3::new(WE, 0.0, W) + offset,
                Vector3::new(1.00, 0.0, 0.25) + offset,
                Vector3::new(WE, 0.0, 0.25) + offset,
            );
            add_l(
                st,
                Vector3::new(1.00, 0.0, 0.75) + offset,
                Vector3::new(WE, 0.0, 0.75) + offset,
                Vector3::new(1.00, 0.0, 1.00) + offset,
                Vector3::new(WE, 0.0, WE) + offset,
                Vector3::new(0.75, 0.0, 1.00) + offset,
                Vector3::new(0.75, 0.0, WE) + offset,
            );
            add_l(
                st,
                Vector3::new(0.25, 0.0, 1.00) + offset,
                Vector3::new(0.25, 0.0, WE) + offset,
                Vector3::new(0.00, 0.0, 1.00) + offset,
                Vector3::new(W, 0.0, WE) + offset,
                Vector3::new(0.00, 0.0, 0.75) + offset,
                Vector3::new(W, 0.0, 0.75) + offset,
            );
        }

        for x in 0..32 {
            for z in 0..32 {
                add_tile(&mut st, Vector3::new(x as f32, 0.0, z as f32));
            }
        }

        let material: Gd<Material> = load("res://assets/materials/fx/mat_fx_lot_buildgrid.tres");
        st.set_material(&material);

        st.commit().unwrap()
    }
}
