//! class: [EntityCollider]
//! desc: A component for entities. This will detect when the entity is clicked.
//!
use godot::classes::{
    Area3D, CollisionShape3D, CylinderShape3D, IArea3D, InputEvent, InputEventMouseButton, Shape3D,
};
use godot::global::MouseButton;
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(base=Area3D)]
pub struct EntityCollider {
    shape_node: Gd<CollisionShape3D>,

    base: Base<Area3D>,
}

#[godot_api]
impl IArea3D for EntityCollider {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            shape_node: CollisionShape3D::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_shape();
    }

    fn input_event(
        &mut self,
        _camera: Option<Gd<Camera3D>>,
        event: Option<Gd<InputEvent>>,
        _event_position: Vector3,
        _normal: Vector3,
        _shape_idx: i32,
    ) {
        let Some(event) = event else {
            return;
        };

        if let Ok(event) = event.clone().try_cast::<InputEventMouseButton>() {
            match event.get_button_index() {
                MouseButton::LEFT => {
                    if event.is_pressed() {
                        self.base_mut().emit_signal("sig_clicked", &[]);
                    }
                }
                _ => return,
            }
        }
    }
}

#[godot_api]
impl EntityCollider {
    #[signal]
    fn sig_clicked();
}

impl EntityCollider {
    fn setup_shape(&mut self) {
        let mut shape = CylinderShape3D::new_gd();
        shape.set_height(1.8);
        shape.set_radius(0.25);
        self.base_mut().translate(Vector3::new(0.0, 0.9, 0.0));

        let mut coll_shape = self.shape_node.clone();
        coll_shape.set_shape(&shape);
        coll_shape.set_name("coll_shape");

        self.base_mut().add_child(&coll_shape);
    }

    pub fn set_shape(&mut self, shape: Gd<Shape3D>) {
        self.shape_node.set_shape(&shape);
    }
}
