// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use godot::classes::control::SizeFlags;
use godot::prelude::*;

use godot::classes::{
    GridContainer, IPanelContainer, Label, PanelContainer, StyleBox, VBoxContainer,
};

#[derive(Debug, GodotClass)]
#[class(base=PanelContainer)]
pub struct UiDebugOvl {
    ui_grid: Gd<GridContainer>,
    /// Keys to display; (lab_k, lab_v)
    ui_labels: HashMap<String, (Gd<Label>, Gd<Label>)>,
    ui_title: Gd<Label>,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for UiDebugOvl {
    fn init(base: Base<Self::Base>) -> Self {
        let mut ui_title = Label::new_alloc();
        ui_title.set_text("This is your debug overlay");

        Self {
            ui_grid: GridContainer::new_alloc(),
            ui_labels: HashMap::new(),
            ui_title,

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_ui();
    }
}

impl UiDebugOvl {
    fn setup_ui(&mut self) {
        let mut ui_title = self.ui_title.clone();
        ui_title.add_theme_constant_override("outline_size", 4);
        ui_title.set_name("ui_title");

        let mut grid = self.ui_grid.clone();
        grid.set_columns(2);
        grid.add_theme_constant_override("h_separation", 16);
        grid.add_theme_constant_override("v_separation", 0);
        grid.set_name("grid");

        let mut vbox = VBoxContainer::new_alloc();
        vbox.add_child(&ui_title);
        vbox.add_child(&grid);
        vbox.set_name("vbox");

        let stylebox: Gd<StyleBox> = load("res://assets/ui/theme_lunacy/stylebox_debugpanel.tres");

        self.base_mut().add_child(&vbox);
        self.base_mut()
            .add_theme_stylebox_override("panel", &stylebox);
        self.base_mut().set_h_size_flags(SizeFlags::SHRINK_BEGIN);
        self.base_mut().set_v_size_flags(SizeFlags::SHRINK_BEGIN);
    }

    pub fn add_key(&mut self, k: String, v: &str) {
        const FONT_SIZE: i32 = 14;
        const OUTLINE_SIZE: i32 = 3;

        let mut lab_k = Label::new_alloc();
        lab_k.set_text(&k);
        lab_k.add_theme_font_size_override("font_size", FONT_SIZE);
        lab_k.add_theme_constant_override("outline_size", OUTLINE_SIZE);
        lab_k.set_name(format!("lab_k_{k}").as_str());

        let mut lab_v = Label::new_alloc();
        lab_v.set_text(v);
        lab_v.add_theme_font_size_override("font_size", FONT_SIZE);
        lab_v.add_theme_constant_override("outline_size", OUTLINE_SIZE);
        lab_v.set_name(format!("lab_v_{k}").as_str());

        self.ui_grid.add_child(&lab_k);
        self.ui_grid.add_child(&lab_v);
        self.ui_labels.insert(k, (lab_k, lab_v));
    }

    pub fn remove_key(&mut self, k: &str) {
        let Some(labels) = self.ui_labels.remove(k) else {
            return;
        };
        self.ui_grid.remove_child(&labels.0);
        self.ui_grid.remove_child(&labels.1);
        labels.0.free();
        labels.1.free();
    }

    pub fn set_value(&mut self, k: &str, v: &str) {
        let Some(labels) = self.ui_labels.get_mut(k) else {
            return;
        };
        labels.1.set_text(v);
    }

    pub fn set_title(&mut self, text: &str) {
        self.ui_title.set_text(text);
        self.ui_title.set_visible(!text.is_empty());
    }
}
