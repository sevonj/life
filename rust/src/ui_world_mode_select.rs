// SPDX-License-Identifier: LGPL-3.0-or-later
//! Class: [UiPersonBioPanel]
//! Desc: A large panel that shows a person's info.
//!
use godot::classes::{Button, IMarginContainer, MarginContainer, VBoxContainer};
use godot::prelude::*;

use crate::World;
use crate::WorldViewMode;

const MIN_W: f32 = 64.0;

#[derive(Debug, GodotClass)]
#[class(base=MarginContainer)]
pub struct UiWorldModeSelectOld {
    world: Option<Gd<World>>,

    button_play: Gd<Button>,
    button_buy: Gd<Button>,
    button_build: Gd<Button>,

    base: Base<MarginContainer>,
}

#[godot_api]
impl IMarginContainer for UiWorldModeSelectOld {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            world: None,

            button_play: Button::new_alloc(),
            button_buy: Button::new_alloc(),
            button_build: Button::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_ui();
    }

    fn process(&mut self, _delta: f64) {
        let Some(world) = &self.world else {
            return;
        };

        match world.bind().view_mode() {
            WorldViewMode::Build => {
                self.button_play.set_pressed_no_signal(false);
                self.button_buy.set_pressed_no_signal(false);
                self.button_build.set_pressed_no_signal(true);
            }
            WorldViewMode::Buy => {
                self.button_play.set_pressed_no_signal(false);
                self.button_buy.set_pressed_no_signal(true);
                self.button_build.set_pressed_no_signal(false);
            }
            WorldViewMode::Play => {
                self.button_play.set_pressed_no_signal(true);
                self.button_buy.set_pressed_no_signal(false);
                self.button_build.set_pressed_no_signal(false);
            }
        }
    }
}

#[godot_api]
impl UiWorldModeSelectOld {
    #[func]
    fn on_set_mode_play(&mut self) {
        if let Some(world) = &mut self.world {
            world.bind_mut().set_view_mode(WorldViewMode::Play);
        }
    }

    #[func]
    fn on_set_mode_buy(&mut self) {
        if let Some(world) = &mut self.world {
            world.bind_mut().set_view_mode(WorldViewMode::Buy);
        }
    }

    #[func]
    fn on_set_mode_build(&mut self) {
        if let Some(world) = &mut self.world {
            world.bind_mut().set_view_mode(WorldViewMode::Build);
        }
    }
}

impl UiWorldModeSelectOld {
    fn setup_ui(&mut self) {
        let this_gd = &self.to_gd();

        self.button_play.set_name("button_play");
        self.button_play.set_text("Play");
        self.button_play.set_toggle_mode(true);
        self.button_play.set_tooltip_text("Enter play mode");
        self.button_play
            .connect("pressed", &this_gd.callable("on_set_mode_play"));

        self.button_buy.set_name("button_buy");
        self.button_buy.set_text("Buy");
        self.button_buy.set_toggle_mode(true);
        self.button_buy.set_disabled(true);
        self.button_buy
            .set_tooltip_text("Buy mode is not implemented.");
        self.button_buy
            .connect("pressed", &this_gd.callable("on_set_mode_buy"));

        self.button_build.set_name("button_build");
        self.button_build.set_text("Build");
        self.button_build.set_toggle_mode(true);
        self.button_build.set_tooltip_text("Enter build mode");
        self.button_build
            .connect("pressed", &this_gd.callable("on_set_mode_build"));

        let mut vbox = VBoxContainer::new_alloc();
        vbox.add_child(&self.button_play);
        vbox.add_child(&self.button_buy);
        vbox.add_child(&self.button_build);

        self.base_mut().add_child(&vbox);
        self.base_mut()
            .set_custom_minimum_size(Vector2::new(MIN_W, 32.0));
    }

    pub fn connect_world(&mut self, world: Gd<World>) {
        self.world = Some(world)
    }
}
