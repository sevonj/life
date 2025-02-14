use std::collections::{HashMap, HashSet};

use godot::{
    classes::{mesh::PrimitiveType, ArrayMesh, Material, SurfaceTool},
    global::deg_to_rad,
    prelude::*,
};

const WALL_THICKNESS: f32 = 0.15;

#[derive(Debug, Clone)]
pub struct Wall {
    /// For now, keep span length 1!
    span: (Vector2i, Vector2i),
}

impl Wall {
    pub fn new(start: Vector2i, end: Vector2i) -> Result<Self, String> {
        let span_relative = end - start;
        if span_relative == Vector2i::ZERO {
            return Err("Span length can't be zero!".into());
        }
        if span_relative.x != 0 && span_relative.y != 0 {
            return Err("Diagonal walls are not supported (yet)!".into());
        }
        if start.x < 0 || start.y < 0 || end.x < 0 || end.y < 0 {
            return Err("You passed negative coords!".into());
        }
        // No, this isn't a mistake. The wall grid is 33x33.
        if start.x > 32 || start.y > 32 || end.x > 32 || end.y > 32 {
            return Err("Span coords are out of bounds!".into());
        }

        if span_relative.x + span_relative.y != 1 {
            return Err("For now, keep span length 1".into());
        }

        Ok(Self { span: (start, end) })
    }

    pub fn span(&self) -> (Vector2i, Vector2i) {
        self.span
    }
}

#[derive(Debug, Clone)]
pub struct Walls {
    walls: HashMap<(Vector2i, Vector2i), Wall>,
}

impl Walls {
    pub fn with_test_layout() -> Self {
        let wall_vec = vec![
            Wall::new(Vector2i::new(4, 8), Vector2i::new(5, 8)).unwrap(),
            Wall::new(Vector2i::new(4, 8), Vector2i::new(4, 9)).unwrap(),
            //LotWall::new(Vector2i::new(4, 9), Vector2i::new(4, 10)).unwrap(),
            Wall::new(Vector2i::new(4, 10), Vector2i::new(4, 11)).unwrap(),
            Wall::new(Vector2i::new(4, 11), Vector2i::new(4, 12)).unwrap(),
            Wall::new(Vector2i::new(4, 12), Vector2i::new(4, 13)).unwrap(),
            Wall::new(Vector2i::new(4, 13), Vector2i::new(4, 14)).unwrap(),
            Wall::new(Vector2i::new(4, 14), Vector2i::new(4, 15)).unwrap(),
            Wall::new(Vector2i::new(4, 15), Vector2i::new(4, 16)).unwrap(),
            Wall::new(Vector2i::new(4, 16), Vector2i::new(5, 16)).unwrap(),
        ];

        let mut walls = HashMap::new();
        for wall in wall_vec {
            walls.insert(wall.span(), wall);
        }

        let this = Self { walls };

        this
    }

    pub fn add_wall(&mut self, wall: Wall) {
        let k = Self::span_sorted(wall.span());
        self.walls.insert(k, wall);
    }

    pub fn remove_wall(&mut self, span: (Vector2i, Vector2i)) {
        let k = Self::span_sorted(span);
        self.walls.remove(&k);
    }

    /// Smaller X first. If X are equal, smaller Y first.
    const fn span_sorted(span: (Vector2i, Vector2i)) -> (Vector2i, Vector2i) {
        if span.0.x < span.1.x {
            return (span.0, span.1);
        }
        if span.0.x == span.1.x {
            if span.0.y < span.1.y {
                return (span.0, span.1);
            }
        }
        (span.1, span.0)
    }

    pub fn to_mesh(&self) -> Gd<ArrayMesh> {
        // key: coordinate
        // val: connected neighbor coordinates
        let mut connections = HashMap::new();

        for wall in self.walls.values() {
            let a = wall.span().0;
            let b = wall.span().1;
            if !connections.contains_key(&a) {
                connections.insert(a, HashSet::new());
            }
            if !connections.contains_key(&b) {
                connections.insert(b, HashSet::new());
            }
            connections.get_mut(&a).unwrap().insert(b);
            connections.get_mut(&b).unwrap().insert(a);
        }

        let mut st = SurfaceTool::new_gd();
        st.begin(PrimitiveType::TRIANGLE_STRIP);

        fn add_quad(
            st: &mut SurfaceTool,
            normal: Vector3,
            a: Vector3,
            b: Vector3,
            c: Vector3,
            d: Vector3,
        ) {
            // Duplicate for degen triangle
            st.set_normal(normal);
            st.set_uv(Vector2::new(0.0, 0.0));
            st.add_vertex(a);

            st.set_normal(normal);
            st.set_uv(Vector2::new(0.0, 0.0));
            st.add_vertex(a);

            st.set_normal(normal);
            st.set_uv(Vector2::new(1.0, 0.0));
            st.add_vertex(b);

            st.set_normal(normal);
            st.set_uv(Vector2::new(0.0, 1.0));
            st.add_vertex(c);

            st.set_normal(normal);
            st.set_uv(Vector2::new(1.0, 1.0));
            st.add_vertex(d);

            // Duplicate for degen triangle
            st.set_normal(normal);
            st.set_uv(Vector2::new(1.0, 1.0));
            st.add_vertex(d);
        }

        for a in connections.keys() {
            for b in &connections[a] {
                let v_a = Vector2::new(a.x as f32, a.y as f32);
                let v_b = Vector2::new(b.x as f32, b.y as f32);
                let dir = Vector3::new(v_b.x - v_a.x, 0.0, v_b.y - v_a.y).normalized();

                let normal_2d = (v_b - v_a).normalized().rotated(deg_to_rad(90.0) as f32);
                let normal = Vector3::new(normal_2d.x, 0.0, normal_2d.y);

                let v_0 = Vector3::new(v_a.x, 0.0, v_a.y);
                let v_1 = Vector3::new(v_b.x, 0.0, v_b.y);
                let v_2 = Vector3::new(v_a.x, 2.0, v_a.y);
                let v_3 = Vector3::new(v_b.x, 2.0, v_b.y);

                let offset = normal * WALL_THICKNESS / 2.0;

                // wall
                add_quad(
                    &mut st,
                    normal,
                    v_0 + offset,
                    v_1 + offset,
                    v_2 + offset,
                    v_3 + offset,
                );

                // end cap
                add_quad(
                    &mut st,
                    dir,
                    v_1 + offset,
                    v_1 - offset,
                    v_3 + offset,
                    v_3 - offset,
                );
            }
        }

        // top cap
        for wall in self.walls.values() {
            let a = wall.span().0;
            let b = wall.span().1;
            let v_a = Vector2::new(a.x as f32, a.y as f32);
            let v_b = Vector2::new(b.x as f32, b.y as f32);

            let normal_2d = (v_b - v_a).normalized().rotated(deg_to_rad(90.0) as f32);
            let normal = Vector3::new(normal_2d.x, 0.0, normal_2d.y);

            let v_2 = Vector3::new(v_a.x, 2.0, v_a.y);
            let v_3 = Vector3::new(v_b.x, 2.0, v_b.y);

            let offset = normal * WALL_THICKNESS / 2.0;

            add_quad(
                &mut st,
                Vector3::UP,
                v_2 + offset,
                v_3 + offset,
                v_2 - offset,
                v_3 - offset,
            );
        }

        let material: Gd<Material> = load("res://assets/materials/mat_wall_bare.tres");
        st.set_material(&material);

        let mesh = st.commit();

        mesh.unwrap()
    }
}
