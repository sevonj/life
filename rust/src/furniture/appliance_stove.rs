use godot::{
    classes::{BoxShape3D, Shape3D},
    prelude::*,
};

use crate::{action_advertisement::ActionAdvertisementStat, ActionAdvertisement, EntityCollider};

use super::Furniture;

#[derive(Debug, GodotClass)]
#[class(base=Node3D)]
pub struct ApplianceStove {
    node_visuals: Gd<Node3D>,
    node_collider: Gd<EntityCollider>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for ApplianceStove {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            node_visuals: Node3D::new_alloc(),
            node_collider: EntityCollider::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_visuals();
        self.setup_collider();
    }
}

impl Furniture for ApplianceStove {
    fn available_actions(&self) -> Vec<ActionAdvertisement> {
        vec![ActionAdvertisement {
            action_key: "make_food".into(),
            source_node: self.to_gd().upcast::<Node3D>(),
            stats: vec![
                ActionAdvertisementStat {
                    key: "hunger".into(),
                    value: 10,
                },
                ActionAdvertisementStat {
                    key: "bladder".into(),
                    value: -2,
                },
            ],
        }]
    }
}

#[godot_api]
impl ApplianceStove {
    /// This furniture was selected.
    #[signal]
    fn sig_selected(furniture: Gd<ApplianceStove>);

    /// When [node_collider] is clicked.
    #[func]
    fn on_click(&mut self) {
        let this = self.to_gd().to_variant();
        self.base_mut().emit_signal("sig_selected", &[this]);
    }
}

impl ApplianceStove {
    fn setup_visuals(&mut self) {
        let model_packed: Gd<PackedScene> = load("res://assets/furniture/mdl_appliance_stove_1x1_001.blend");
        let mut model = model_packed.instantiate().unwrap();
        model.set_name("model");

        let mut node_visuals = self.node_visuals.clone();
        node_visuals.add_child(&model);
        node_visuals.set_name("visuals");

        self.base_mut().add_child(&node_visuals);
    }

    fn setup_collider(&mut self) {
        let mut shape = BoxShape3D::new_gd();
        shape.set_size(Vector3::new(1.0, 1.0, 1.0));
        let shape = shape.upcast::<Shape3D>();

        let mut node_collider = self.node_collider.clone();
        node_collider.connect("sig_clicked", &self.to_gd().callable("on_click"));
        node_collider.bind_mut().set_shape(shape);
        node_collider.set_position(Vector3::new(0.0, 0.5, 0.0));
        node_collider.set_name("collider");

        self.base_mut().add_child(&node_collider);
    }
}
