use godot::{classes::MeshInstance3D, prelude::*};

use super::{tool_helper, ToolGizmo};
use crate::lot_data::{self, Wall};

#[derive(Debug, Default)]
enum WallToolMode {
    #[default]
    Add,
    Remove,
}

#[derive(Debug, GodotClass)]
#[class(base=Node)]
pub struct WallTool {
    wall_data: lot_data::Walls,
    tool_mode: WallToolMode,
    /// In-progress tool operation, if any.
    tool_span: Option<(Vector2i, Vector2i)>,

    wall_preview: Gd<MeshInstance3D>,

    tool_span_start_gizmo: Gd<ToolGizmo>,
    tool_hover_gizmo: Gd<ToolGizmo>,

    base: Base<Node>,
}

#[godot_api]
impl INode for WallTool {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            wall_data: lot_data::Walls::with_test_layout(),
            tool_mode: WallToolMode::default(),
            tool_span: None,

            wall_preview: MeshInstance3D::new_alloc(),

            tool_span_start_gizmo: ToolGizmo::new_alloc(),
            tool_hover_gizmo: ToolGizmo::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_gizmos();
        self.setup_scene();
    }

    fn process(&mut self, _delta: f64) {
        let _input = Input::singleton();

        self.process_tool();
        self.process_tool_span_start_gizmo();
        self.process_tool_hover_gizmo();
    }
}

impl WallTool {
    fn setup_gizmos(&mut self) {
        let mut tool_span_start_gizmo = self.tool_span_start_gizmo.clone();
        tool_span_start_gizmo.set_name("tool_span_start_gizmo");

        let mut tool_hover_gizmo = self.tool_hover_gizmo.clone();
        tool_hover_gizmo.set_name("tool_hover_gizmo");

        self.base_mut().add_child(&tool_span_start_gizmo);
        self.base_mut().add_child(&tool_hover_gizmo);
    }

    fn setup_scene(&mut self) {
        let mut wall_preview = self.wall_preview.clone();
        wall_preview.set_name("wall_preview");

        self.base_mut().add_child(&wall_preview);
    }

    fn process_tool(&mut self) {
        let input = Input::singleton();

        let hover_coord_opt = self
            .base()
            .get_viewport()
            .map(|f| tool_helper::hovered_wall_grid_coord(f))
            .flatten();

        // No tool operation
        let Some(mut span) = self.tool_span else {
            // Begin tool operation?
            if input.is_action_just_pressed("tool_use") {
                if let Some(coord) = hover_coord_opt {
                    self.tool_span = Some((coord, coord));
                }
            }
            return;
        };

        // Continue tool operation
        if let Some(coord) = hover_coord_opt {
            let to_x = Vector2i::new(coord.x, span.0.y);
            let to_y = Vector2i::new(span.0.x, coord.y);

            if coord.distance_to(to_x) > coord.distance_to(to_y) {
                span.1 = to_y;
            } else {
                span.1 = to_x;
            }

            self.tool_span = Some(span);
        };

        // Cancel tool operation
        if input.is_action_just_pressed("tool_cancel") {
            self.tool_span = None;
            return;
        }

        // Commit tool operation
        if input.is_action_just_pressed("tool_commit") {
            if hover_coord_opt.is_some() {
                self.tool_span = None;
                self.add_span(span);
                let mesh = self.wall_data.to_mesh();
                self.wall_preview.set_mesh(&mesh);
                return;
            }
        }
    }

    fn process_tool_span_start_gizmo(&mut self) {
        if let Some(span) = self.tool_span {
            self.tool_span_start_gizmo.show();
            self.tool_span_start_gizmo.set_position(Vector3::new(
                span.0.x as f32,
                0.4,
                span.0.y as f32,
            ));
        } else {
            self.tool_span_start_gizmo.hide();
        }
    }

    fn process_tool_hover_gizmo(&mut self) {
        let input = Input::singleton();
        if input.is_action_pressed("camera_mod_rotate") {
            return;
        };
        if input.is_action_pressed("camera_mod_move") {
            return;
        };

        let hover_coord_opt = self
            .base()
            .get_viewport()
            .map(|f| tool_helper::hovered_wall_grid_coord(f))
            .flatten();

        let Some(hover_coord) = hover_coord_opt else {
            self.tool_hover_gizmo.hide();
            return;
        };
        self.tool_hover_gizmo.show();

        // Free hover coord or locked to span end.
        let pos = match self.tool_span {
            Some(span) => Vector3::new(span.1.x as f32, 0.4, span.1.y as f32),
            None => Vector3::new(hover_coord.x as f32, 0.4, hover_coord.y as f32),
        };
        self.tool_hover_gizmo.set_position(pos);
    }

    fn add_span(&mut self, span: (Vector2i, Vector2i)) {
        let relative = span.1 - span.0;
        let mut wall_segments = vec![];

        if relative.x == 0 {
            let y_min = span.0.y.min(span.1.y);
            let y_max = span.0.y.max(span.1.y);
            for y in y_min..y_max {
                wall_segments.push(Wall::new(
                    Vector2i::new(span.0.x, y),
                    Vector2i::new(span.0.x, y + 1),
                ));
            }
        } else {
            let x_min = span.0.x.min(span.1.x);
            let x_max = span.0.x.max(span.1.x);
            for x in x_min..x_max {
                wall_segments.push(Wall::new(
                    Vector2i::new(x, span.0.y),
                    Vector2i::new(x + 1, span.0.y),
                ));
            }
        }

        for wall in wall_segments {
            self.add_wall(wall.unwrap());
        }
    }

    fn add_wall(&mut self, wall: Wall) {
        self.wall_data.add_wall(wall);
    }
}
