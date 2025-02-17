// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! class: [CameraRigOrbit]
//! desc: An orbiting camera controller
//!
use godot::classes::Engine;
use godot::classes::{input::MouseMode, node::ProcessMode, InputEvent, InputEventMouseMotion};
use godot::global::deg_to_rad;
use godot::prelude::*;

const MIN_DISTANCE: f32 = 5.0;
const MAX_DISTANCE: f32 = 100.0;

#[derive(Debug, GodotClass)]
#[class(base=Node3D)]
pub struct CameraRigOrbit {
    pivot: Gd<Node3D>,
    camera: Gd<Camera3D>,

    distance: f32,
    stashed_mouse_pos: Option<Vector2>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for CameraRigOrbit {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            pivot: Node3D::new_alloc(),
            camera: Camera3D::new_alloc(),

            distance: 10.0,
            stashed_mouse_pos: None,

            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_process_mode(ProcessMode::ALWAYS);

        self.setup_camera();

        // A decent start at the center of the 32x32 lot
        self.base_mut().set_position(Vector3::new(16.0, 0.0, 16.0));
        self.base_mut().rotate_y(deg_to_rad(45.0) as f32);
        self.pivot.rotate_x(deg_to_rad(-60.0) as f32);
    }

    fn process(&mut self, delta: f64) {
        let delta = delta / Engine::singleton().get_time_scale();

        self.process_input(delta);

        self.camera.set_position(Vector3::BACK * self.distance);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let input = Input::singleton();

        if let Ok(event) = event.clone().try_cast::<InputEventMouseMotion>() {
            if input.is_action_pressed("camera_mod_rotate") {
                let vec = event.get_relative();
                self.base_mut().rotate_y(vec.x * -0.005);

                let mut pivot_rot = self.pivot.get_rotation_degrees();
                pivot_rot.x += vec.y * -0.2;
                pivot_rot.x = pivot_rot.x.clamp(-90.0, -2.0);
                self.pivot.set_rotation_degrees(pivot_rot);
            } else if input.is_action_pressed("camera_mod_move") {
            }
        }
    }
}

impl CameraRigOrbit {
    fn setup_camera(&mut self) {
        self.camera.set_name("camera3d");

        let mut pivot = self.pivot.clone();
        pivot.add_child(&self.camera);
        pivot.set_name("pivot");

        self.base_mut().add_child(&pivot);
    }

    fn process_input(&mut self, delta: f64) {
        let mut input = Input::singleton();

        if input.is_action_just_pressed("camera_zoom_in") {
            self.zoom_in();
        }
        if input.is_action_just_pressed("camera_zoom_out") {
            self.zoom_out();
        }
        if input.is_action_pressed("camera_mod_rotate") {
            if input.get_mouse_mode() == MouseMode::VISIBLE {
                if let Some(vport) = self.base().get_viewport() {
                    self.stashed_mouse_pos = Some(vport.get_mouse_position());
                }
            }
            input.set_mouse_mode(MouseMode::CAPTURED);
        } else {
            input.set_mouse_mode(MouseMode::VISIBLE);
            if let Some(pos) = self.stashed_mouse_pos {
                input.warp_mouse(pos);
                self.stashed_mouse_pos = None;
            }
        }

        let move_x = input.get_axis("camera_move_left", "camera_move_right");
        let move_z = input.get_axis("camera_move_forward", "camera_move_back");
        let mut delta_move = Vector3::new(move_x, 0.0, move_z) * delta as f32 * self.distance;
        delta_move = delta_move.rotated(Vector3::UP, self.base().get_rotation().y);

        let mut pos = self.base().get_position();
        pos += delta_move;
        pos = pos.clamp(Vector3::ZERO, Vector3::ONE * 32.0);

        self.base_mut().set_position(pos);
    }

    fn zoom_in(&mut self) {
        self.distance *= 1.0 / 1.2;
        self.distance = self.distance.clamp(MIN_DISTANCE, MAX_DISTANCE);
    }

    fn zoom_out(&mut self) {
        self.distance *= 1.2;
        self.distance = self.distance.clamp(MIN_DISTANCE, MAX_DISTANCE);
    }
}
