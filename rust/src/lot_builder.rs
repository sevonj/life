use godot::classes::{mesh::PrimitiveType, ArrayMesh, Material, MeshInstance3D, SurfaceTool};
use godot::classes::{CsgBox3D, CsgSphere3D};
use godot::prelude::*;

#[derive(Debug)]
enum LotBuilderTool {
    None,
    WallBuild(Option<(Vector2i, Vector2i)>),
    WallRemove,
}

#[derive(Debug, GodotClass)]
#[class(base=Node)]
pub struct LotBuilder {
    grid: Gd<MeshInstance3D>,
    grid_hover_indicator: Gd<CsgSphere3D>,

    tool: LotBuilderTool,
    tool_helper: Gd<CsgSphere3D>,

    // Parent of temporary wall graphics
    wall_temp_root: Gd<Node3D>,

    base: Base<Node>,
}

#[godot_api]
impl INode for LotBuilder {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            grid: MeshInstance3D::new_alloc(),
            grid_hover_indicator: CsgSphere3D::new_alloc(),

            tool: LotBuilderTool::WallBuild(None),
            tool_helper: CsgSphere3D::new_alloc(),

            wall_temp_root: Node3D::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_scene()
    }

    fn process(&mut self, _delta: f64) {
        self.process_grid_hover_indicator();
        self.process_tool();
        self.process_tool_helper();
    }
}

impl LotBuilder {
    fn setup_scene(&mut self) {
        let mut grid = self.grid.clone();
        grid.set_mesh(&self.build_lot_grid());
        grid.set_position(Vector3::UP * 0.2);
        grid.set_name("grid");

        let mut grid_hover_indicator = self.grid_hover_indicator.clone();
        grid_hover_indicator.set_radius(0.2);
        grid_hover_indicator.set_smooth_faces(false);
        grid_hover_indicator.set_radial_segments(4);
        grid_hover_indicator.set_rings(2);
        let grid_hover_indicator_mat: Gd<Material> =
            load("res://assets/materials/fx/mat_fx_uvcube.tres");
        grid_hover_indicator.set_material(&grid_hover_indicator_mat);
        grid_hover_indicator.set_name("grid_hover_indicator");

        let mut tool_helper = self.tool_helper.clone();
        tool_helper.set_radius(0.2);
        tool_helper.set_smooth_faces(false);
        tool_helper.set_radial_segments(4);
        tool_helper.set_rings(2);
        let grid_hover_indicator_mat: Gd<Material> =
            load("res://assets/materials/fx/mat_fx_uvcube2.tres");
        tool_helper.set_material(&grid_hover_indicator_mat);
        tool_helper.set_name("tool_helper");

        let mut wall_temp_root = self.wall_temp_root.clone();
        wall_temp_root.set_name("wall_temp_root");

        self.base_mut().add_child(&grid);
        self.base_mut().add_child(&grid_hover_indicator);
        self.base_mut().add_child(&tool_helper);
        self.base_mut().add_child(&wall_temp_root);
    }

    fn hovered_grid_coord(&self) -> Option<Vector2i> {
        let Some(viewport) = self.base().get_viewport() else {
            return None;
        };
        let Some(camera) = viewport.get_camera_3d() else {
            return None;
        };

        let mouse_pos = viewport.get_mouse_position();
        let mouse_origin = camera.project_ray_origin(mouse_pos);
        let mouse_normal = camera.project_ray_normal(mouse_pos);

        let floor_height = 0.0;
        let floor = Plane::new(Vector3::UP, floor_height);

        let Some(hover_pos) =
            floor.intersect_ray(mouse_origin, mouse_origin + mouse_normal * 1024.0)
        else {
            return None;
        };

        let coord = Vector2i::new(hover_pos.x.round() as i32, hover_pos.z.round() as i32);
        if coord.x < 0 || coord.y < 0 || coord.x > 32 || coord.y > 32 {
            return None;
        }
        Some(coord)
    }

    fn process_tool(&mut self) {
        let input = Input::singleton();

        let hovered_grid_coord = self.hovered_grid_coord();

        match self.tool {
            LotBuilderTool::None => return,
            LotBuilderTool::WallBuild(span_opt) => {
                if input.is_action_just_pressed("ui_accept") {
                    if span_opt.is_none() {
                        if let Some(coord) = hovered_grid_coord {
                            self.tool = LotBuilderTool::WallBuild(Some((coord, coord)));
                        }
                    }
                }

                if let Some(mut span) = span_opt {
                    if let Some(coord) = hovered_grid_coord {
                        let to_x = Vector2i::new(coord.x, span.0.y);
                        let to_y = Vector2i::new(span.0.x, coord.y);

                        if coord.distance_to(to_x) > coord.distance_to(to_y) {
                            span.1 = to_y;
                        } else {
                            span.1 = to_x;
                        }

                        self.tool = LotBuilderTool::WallBuild(Some(span));
                    };

                    if input.is_action_just_pressed("ui_accept") {
                        self.tool = LotBuilderTool::WallBuild(None);
                        if let Some(coord) = hovered_grid_coord {
                            self.add_wall(span);
                        }
                    }
                }
            }
            LotBuilderTool::WallRemove => todo!(),
        }
    }

    fn process_grid_hover_indicator(&mut self) {
        let input = Input::singleton();

        if input.is_action_pressed("camera_mod_rotate") {
            self.grid_hover_indicator.hide();
            return;
        };
        if input.is_action_pressed("camera_mod_move") {
            self.grid_hover_indicator.hide();
            return;
        };
        let Some(coord) = self.hovered_grid_coord() else {
            self.grid_hover_indicator.hide();
            return;
        };

        self.grid_hover_indicator.show();

        match self.tool {
            LotBuilderTool::None => {
                self.grid_hover_indicator.set_position(Vector3::new(
                    coord.x as f32,
                    0.4,
                    coord.y as f32,
                ));
            }
            LotBuilderTool::WallBuild(span_opt) => {
                if let Some(span) = span_opt {
                    self.grid_hover_indicator.set_position(Vector3::new(
                        span.1.x as f32,
                        0.4,
                        span.1.y as f32,
                    ));
                } else {
                    self.grid_hover_indicator.set_position(Vector3::new(
                        coord.x as f32,
                        0.4,
                        coord.y as f32,
                    ));
                }
            }
            LotBuilderTool::WallRemove => {
                self.grid_hover_indicator.set_position(Vector3::new(
                    coord.x as f32,
                    0.4,
                    coord.y as f32,
                ));
            }
        }
    }

    fn process_tool_helper(&mut self) {
        match self.tool {
            LotBuilderTool::None => {
                self.tool_helper.hide();
                return;
            }
            LotBuilderTool::WallBuild(span_opt) => {
                let Some(span) = span_opt else {
                    self.tool_helper.hide();
                    return;
                };

                self.tool_helper
                    .set_position(Vector3::new(span.0.x as f32, 0.4, span.0.y as f32));
                self.tool_helper.show();
            }
            LotBuilderTool::WallRemove => todo!(),
        }
    }

    fn add_wall(&mut self, span: (Vector2i, Vector2i)) {
        let mut temp_wall = CsgBox3D::new_alloc();
        let material: Gd<Material> = load("res://assets/materials/mat_wall_bare.tres");
        temp_wall.set_material(&material);
        self.wall_temp_root.add_child(&temp_wall);

        let relative = (span.0 - span.1).abs();
        if relative.x > relative.y {
            temp_wall.set_size(Vector3::new(relative.x as f32, 2.0, 0.15));

            let x = span.0.x.min(span.1.x) as f32 + relative.x as f32 / 2.0;
            let z = span.0.y.min(span.1.y) as f32;
            temp_wall.set_position(Vector3::new(x, 1.0, z));
        } else {
            temp_wall.set_size(Vector3::new(0.15, 2.0, relative.y as f32));

            let x = span.0.x.min(span.1.x) as f32;
            let z = span.0.y.min(span.1.y) as f32 + relative.y as f32 / 2.0;
            temp_wall.set_position(Vector3::new(x, 1.0, z));
        }
    }

    fn build_lot_grid(&self) -> Gd<ArrayMesh> {
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
