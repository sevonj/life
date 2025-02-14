use godot::classes::CsgBox3D;
use godot::classes::Material;
use godot::prelude::*;

use super::BuilderGrid;
use super::WallTool;

#[derive(Debug, Default)]
enum LotBuilderTool {
    #[default]
    None,
    Wall(Gd<WallTool>),
}

#[derive(Debug, GodotClass)]
#[class(base=Node)]
pub struct LotBuilder {
    grid: Gd<BuilderGrid>,

    tool: LotBuilderTool,

    // Parent of temporary wall graphics
    wall_temp_root: Gd<Node3D>,

    base: Base<Node>,
}

#[godot_api]
impl INode for LotBuilder {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            grid: BuilderGrid::new_alloc(),

            tool: LotBuilderTool::None,

            wall_temp_root: Node3D::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_scene();

        let wall_tool = WallTool::new_alloc();
        self.base_mut().add_child(&wall_tool);
        self.tool = LotBuilderTool::Wall(wall_tool);
    }

    fn process(&mut self, _delta: f64) {}
}

impl LotBuilder {
    fn setup_scene(&mut self) {
        let mut grid = self.grid.clone();
        grid.set_position(Vector3::UP * 0.2);
        grid.set_name("grid");

        let mut wall_temp_root = self.wall_temp_root.clone();
        wall_temp_root.set_name("wall_temp_root");

        self.base_mut().add_child(&grid);
        self.base_mut().add_child(&wall_temp_root);
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
}
