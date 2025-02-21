// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
// SPDX-FileCopyrightText: 2025 sevonj
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::Engine;
use godot::classes::{input::MouseMode, node::ProcessMode, InputEvent, InputEventMouseMotion};
use godot::global::deg_to_rad;
use godot::prelude::*;

use crate::CameraCursorGizmo;

const MIN_DISTANCE: f32 = 5.0;
const MAX_DISTANCE: f32 = 100.0;

/// A multipurpose orbiting camera controller
#[derive(Debug, GodotClass)]
#[class(base=Node3D)]
pub struct CameraRigOrbit {
    lock_vertical: bool,

    pivot: Gd<Node3D>,
    camera: Gd<Camera3D>,
    cursor_gizmo: Gd<CameraCursorGizmo>,

    distance: f32,
    stashed_mouse_pos: Option<Vector2>,
    mouse_delta: Vector2,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for CameraRigOrbit {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            lock_vertical: true,

            pivot: Node3D::new_alloc(),
            camera: Camera3D::new_alloc(),
            cursor_gizmo: CameraCursorGizmo::new_alloc(),

            distance: 10.0,
            stashed_mouse_pos: None,
            mouse_delta: Vector2::ZERO,

            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_process_mode(ProcessMode::ALWAYS);

        self.setup_camera();
        self.setup_ui();

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
        if let Ok(event) = event.clone().try_cast::<InputEventMouseMotion>() {
            self.mouse_delta = event.get_relative();
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

    fn setup_ui(&mut self) {
        let cursor_gizmo = self.cursor_gizmo.clone();
        self.base_mut().add_child(&cursor_gizmo);
    }

    fn process_input(&mut self, delta: f64) {
        let mut input = Input::singleton();

        #[derive(Debug, PartialEq, Eq)]
        enum CamAction {
            None,
            Rotate,
            Move,
        }

        let action = if !input.is_action_pressed("camera_action") {
            CamAction::None
        } else if input.is_action_pressed("camera_mod_move") {
            CamAction::Move
        } else {
            CamAction::Rotate
        };

        if input.is_action_just_pressed("camera_zoom_in") {
            self.zoom_in();
        } else if input.is_action_just_pressed("camera_zoom_out") {
            self.zoom_out();
        }

        if action != CamAction::None {
            if input.get_mouse_mode() == MouseMode::VISIBLE {
                if let Some(vport) = self.base().get_viewport() {
                    let position = vport.get_mouse_position();
                    self.stashed_mouse_pos = Some(position);
                    self.cursor_gizmo.bind_mut().set_position(position);
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

        if action == CamAction::Rotate {
            let vec = self.mouse_delta;
            self.base_mut().rotate_y(vec.x * -0.005);

            let mut pivot_rot = self.pivot.get_rotation_degrees();
            pivot_rot.x += vec.y * -0.2;
            pivot_rot.x = pivot_rot.x.clamp(-90.0, -2.0);
            self.pivot.set_rotation_degrees(pivot_rot);
        }

        let move_vec = if action == CamAction::Move {
            const SENS: f32 = 0.2;
            if self.lock_vertical {
                Vector3::new(-self.mouse_delta.x * SENS, 0.0, -self.mouse_delta.y * SENS)
            } else {
                let x = self.camera.get_transform().basis.col_a() * -self.mouse_delta.x * SENS;
                let y = self.pivot.get_transform().basis.col_b() * self.mouse_delta.y * SENS;
                x + y
            }
        } else if self.lock_vertical {
            Vector3::new(
                input.get_axis("camera_move_left", "camera_move_right"),
                0.0,
                input.get_axis("camera_move_forward", "camera_move_back"),
            )
        } else {
            Vector3::new(
                input.get_axis("camera_move_left", "camera_move_right"),
                input.get_axis("camera_move_down", "camera_move_up"),
                input.get_axis("camera_move_forward", "camera_move_back"),
            )
        };

        let mut delta_move = move_vec * delta as f32 * self.distance;
        delta_move = delta_move.rotated(Vector3::UP, self.base().get_rotation().y);

        let mut pos = self.base().get_position();
        pos += delta_move;
        pos = pos.clamp(Vector3::ZERO, Vector3::ONE * 32.0);
        self.base_mut().set_position(pos);

        self.mouse_delta = Vector2::ZERO;
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
