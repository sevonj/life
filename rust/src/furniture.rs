// SPDX-License-Identifier: LGPL-3.0-or-later
use godot::{
    classes::{BoxShape3D, Shape3D},
    prelude::*,
};

use crate::{ActionAdvertisement, EntityCollider, Person};

#[derive(Debug, GodotClass)]
#[class(base=Node3D)]
pub struct Furniture {
    actions: Vec<ActionAdvertisement>,
    reserved_by: Option<Gd<Person>>,

    node_visuals: Gd<Node3D>,
    node_collider: Gd<EntityCollider>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Furniture {
    fn init(base: Base<Self::Base>) -> Self {
        godot_error!("Don't use the default constructor! Use ::new() (in rust) instead.");

        let mut coll_box = BoxShape3D::new_gd();
        coll_box.set_size(Vector3::new(1.0, 1.0, 1.0));

        let coll_shape: Gd<Shape3D> = coll_box.upcast::<Shape3D>();
        let coll_offset = Vector3::new(0.0, 0.5, 0.0);

        Self {
            actions: vec![],
            reserved_by: None,

            node_visuals: Self::build_visuals("res://assets/models/mdl_debug_error.glb"),
            node_collider: Self::build_collider(coll_shape, coll_offset),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_visuals();
        self.setup_collider();
    }
}

#[godot_api]
impl Furniture {
    /// This furniture was selected.
    #[signal]
    fn sig_selected(furniture: Gd<Furniture>);

    /// When [node_collider] is clicked.
    #[func]
    fn on_click(&mut self) {
        let this = self.to_gd().to_variant();
        self.base_mut().emit_signal("sig_selected", &[this]);
    }
}

impl Furniture {
    pub fn is_reserved(&self) -> bool {
        self.reserved_by.is_some()
    }

    // fn deserialize() -> Result<Gd<Self>, ()> {}

    /// The constructor
    pub fn new(
        model_path: &str,
        coll_shape: Gd<Shape3D>,
        coll_offset: Vector3,
        actions: Vec<ActionAdvertisement>,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            actions,
            reserved_by: None,

            node_visuals: Self::build_visuals(model_path),
            node_collider: Self::build_collider(coll_shape, coll_offset),

            base,
        })
    }

    fn build_visuals(model_path: &str) -> Gd<Node3D> {
        let model_packed: Gd<PackedScene> = load(model_path);
        let mut model = model_packed.instantiate().unwrap();
        model.set_name("model");

        let mut node_visuals = Node3D::new_alloc();
        node_visuals.add_child(&model);
        node_visuals.set_name("visuals");

        node_visuals
    }

    fn build_collider(coll_shape: Gd<Shape3D>, offset: Vector3) -> Gd<EntityCollider> {
        let mut node_collider = EntityCollider::new_alloc();
        node_collider.bind_mut().set_shape(coll_shape);
        node_collider.set_position(offset);
        node_collider.set_name("collider");

        node_collider
    }

    fn setup_visuals(&mut self) {
        let node_visuals = self.node_visuals.clone();
        self.base_mut().add_child(&node_visuals);
        //     let model_packed: Gd<PackedScene> =
        //     load("res://assets/furniture/mdl_appliance_stove_1x1_001.blend");
        //     let model_packed: Gd<PackedScene> = load("res://assets/models/mdl_debug_error.glb");
        //     let mut model = model_packed.instantiate().unwrap();
        //     model.set_name("model");
        //
        //     let mut node_visuals = self.node_visuals.clone();
        //     node_visuals.add_child(&model);
        //     node_visuals.set_name("visuals");
        //
        //     self.base_mut().add_child(&node_visuals);
    }

    fn setup_collider(&mut self) {
        // Connect collider signal
        let mut node_collider = self.node_collider.clone();
        node_collider.connect("sig_clicked", &self.to_gd().callable("on_click"));
    }

    /// List of actions that a [crate::Person] can perform on this
    pub fn available_actions(&self) -> &Vec<ActionAdvertisement> {
        &self.actions
    }

    pub fn available_actions_mut(&mut self) -> &mut Vec<ActionAdvertisement> {
        &mut self.actions
    }

    pub fn reserve(&mut self, person: Gd<Person>) {
        self.reserved_by = Some(person)
    }

    pub fn unreserve(&mut self) {
        self.reserved_by = None
    }
}
