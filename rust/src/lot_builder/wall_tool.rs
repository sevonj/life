use godot::{classes::MeshInstance3D, prelude::*};

use super::{tool_gizmo::ToolGizmoStyle, tool_helper, LotBuilder, ToolGizmo};
use crate::lot_data::{self, Wall};

#[derive(Debug, Default)]
enum WallToolMode {
    #[default]
    Add,
    Remove,
}

#[derive(Debug, GodotClass)]
#[class(no_init, base=Node)]
pub struct WallTool {
    builder: Gd<LotBuilder>,

    //wall_data: lot_data::Walls,
    tool_mode: WallToolMode,
    /// In-progress tool operation, if any.
    tool_span: Option<(Vector2i, Vector2i)>,
    /// Indicates where the tool would hit if used or committed.
    gizmo_action: Gd<ToolGizmo>,
    /// Visible when [tool_span] is Some.
    gizmo_span_start: Gd<ToolGizmo>,

    wall_preview: Gd<MeshInstance3D>,

    base: Base<Node>,
}

#[godot_api]
impl INode for WallTool {
    /*fn init(base: Base<Self::Base>) -> Self {
        godot_warn!("Loaded wall tool with dummy test data.");

        Self {
            wall_data: lot_data::Walls::with_test_layout(),
            tool_mode: WallToolMode::default(),
            tool_span: None,
            gizmo_action: ToolGizmo::new_alloc(),
            gizmo_span_start: ToolGizmo::new_alloc(),

            wall_preview: MeshInstance3D::new_alloc(),

            base,
        }
    }*/

    fn ready(&mut self) {
        self.setup_gizmos();
        self.setup_scene();

        self.rebuild_mesh();
    }

    fn process(&mut self, _delta: f64) {
        self.process_tool_mode();
        self.process_tool();
        self.process_tool_span_start_gizmo();
        self.process_tool_hover_gizmo();
    }
}

impl WallTool {
    pub fn new(builder: Gd<LotBuilder>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            builder,

            //wall_data: lot_data::Walls::with_test_layout(),
            tool_mode: WallToolMode::default(),
            tool_span: None,
            gizmo_action: ToolGizmo::new_alloc(),
            gizmo_span_start: ToolGizmo::new_alloc(),

            wall_preview: MeshInstance3D::new_alloc(),

            base,
        })
    }

    fn setup_gizmos(&mut self) {
        let mut tool_span_start_gizmo = self.gizmo_span_start.clone();
        tool_span_start_gizmo.set_name("tool_span_start_gizmo");

        let mut tool_hover_gizmo = self.gizmo_action.clone();
        tool_hover_gizmo.set_name("tool_hover_gizmo");

        self.base_mut().add_child(&tool_span_start_gizmo);
        self.base_mut().add_child(&tool_hover_gizmo);
    }

    fn setup_scene(&mut self) {
        let mut wall_preview = self.wall_preview.clone();
        wall_preview.set_name("wall_preview");

        self.base_mut().add_child(&wall_preview);
    }

    fn process_tool_mode(&mut self) {
        // No changing modes with an ongoing operation
        if self.tool_span.is_some() {
            return;
        }

        let input = Input::singleton();
        if input.is_action_pressed("tool_mod_alt") {
            if self.gizmo_action.bind().style() != ToolGizmoStyle::Destructive {
                self.gizmo_action
                    .bind_mut()
                    .set_style(ToolGizmoStyle::Destructive);
            }
            if self.gizmo_span_start.bind().style() != ToolGizmoStyle::Destructive {
                self.gizmo_span_start
                    .bind_mut()
                    .set_style(ToolGizmoStyle::Destructive);
            }
            self.tool_mode = WallToolMode::Remove;
        } else {
            if self.gizmo_action.bind().style() != ToolGizmoStyle::Normal {
                self.gizmo_action
                    .bind_mut()
                    .set_style(ToolGizmoStyle::Normal);
            }
            if self.gizmo_span_start.bind().style() != ToolGizmoStyle::Normal {
                self.gizmo_span_start
                    .bind_mut()
                    .set_style(ToolGizmoStyle::Normal);
            }
            self.tool_mode = WallToolMode::Add;
        };
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
                match self.tool_mode {
                    WallToolMode::Add => {
                        for piece in self.break_span(span) {
                            let wall = Wall::new(piece.0, piece.1).unwrap();
                            self.add_wall(wall);
                        }
                    }
                    WallToolMode::Remove => {
                        for piece in self.break_span(span) {
                            self.remove_wall(piece);
                        }
                    }
                }
                self.rebuild_mesh();
                return;
            }
        }
    }

    fn process_tool_span_start_gizmo(&mut self) {
        if let Some(span) = self.tool_span {
            self.gizmo_span_start.show();
            self.gizmo_span_start
                .set_position(Vector3::new(span.0.x as f32, 0.4, span.0.y as f32));
        } else {
            self.gizmo_span_start.hide();
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
            self.gizmo_action.hide();
            return;
        };
        self.gizmo_action.show();

        let pos = match self.tool_span {
            Some(span) => Vector3::new(span.1.x as f32, 0.4, span.1.y as f32),
            None => Vector3::new(hover_coord.x as f32, 0.4, hover_coord.y as f32),
        };
        self.gizmo_action.set_position(pos);
    }

    /// Break a span into 1-len pieces
    fn break_span(&mut self, span: (Vector2i, Vector2i)) -> Vec<(Vector2i, Vector2i)> {
        let relative = span.1 - span.0;
        let mut span_pieces = vec![];

        if relative.x == 0 {
            let y_min = span.0.y.min(span.1.y);
            let y_max = span.0.y.max(span.1.y);
            for y in y_min..y_max {
                span_pieces.push((Vector2i::new(span.0.x, y), Vector2i::new(span.0.x, y + 1)));
            }
        } else {
            let x_min = span.0.x.min(span.1.x);
            let x_max = span.0.x.max(span.1.x);
            for x in x_min..x_max {
                span_pieces.push((Vector2i::new(x, span.0.y), Vector2i::new(x + 1, span.0.y)));
            }
        }

        span_pieces
    }

    fn rebuild_mesh(&mut self) {
        let mesh = self.builder.bind().wall_data().to_mesh();
        self.wall_preview.set_mesh(&mesh);
    }

    fn add_wall(&mut self, wall: Wall) {
        self.builder.bind_mut().wall_data_mut().add_wall(wall);
    }

    fn remove_wall(&mut self, span: (Vector2i, Vector2i)) {
        self.builder.bind_mut().wall_data_mut().remove_wall(span);
    }
}
