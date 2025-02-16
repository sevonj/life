use godot::classes::MeshInstance3D;
use godot::prelude::*;

use crate::lot_data;

use super::BuilderGrid;
use super::WallTool;

#[derive(Debug, Default)]
enum LotBuilderTool {
    #[default]
    None,
    #[allow(dead_code)]
    Wall(Gd<WallTool>),
}

#[derive(Debug, GodotClass)]
#[class(no_init, base=Node)]
pub struct LotBuilder {
    grid: Gd<BuilderGrid>,
    tool: LotBuilderTool,
    wall_data: lot_data::Walls,
    _wall_mesh: Gd<MeshInstance3D>,

    base: Base<Node>,
}

#[godot_api]
impl INode for LotBuilder {
    fn ready(&mut self) {
        self.setup_scene();

        let wall_tool = WallTool::new(self.to_gd());
        self.base_mut().add_child(&wall_tool);
        self.tool = LotBuilderTool::Wall(wall_tool);
    }

    fn process(&mut self, _delta: f64) {}
}

impl LotBuilder {
    pub fn new(wall_data: lot_data::Walls, wall_mesh: Gd<MeshInstance3D>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            grid: BuilderGrid::new_alloc(),
            tool: LotBuilderTool::None,
            wall_data,
            _wall_mesh: wall_mesh,

            base,
        })
    }

    fn setup_scene(&mut self) {
        let mut grid = self.grid.clone();
        grid.set_position(Vector3::UP * 0.2);
        grid.set_name("grid");

        self.base_mut().add_child(&grid);
    }

    pub fn wall_data(&self) -> &lot_data::Walls {
        &self.wall_data
    }

    pub fn wall_data_mut(&mut self) -> &mut lot_data::Walls {
        &mut self.wall_data
    }
}
