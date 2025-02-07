//! Class: [UiHomeTaskbar]
//! Desc: The large bottom bar in home view
//!
use godot::{
    classes::{
        control::LayoutPreset, HBoxContainer, IPanelContainer, PanelContainer, StyleBoxTexture,
    },
    prelude::*,
};

use crate::{ui_person_needs_panel::UiPersonNeedsPanel, Person, UiPersonBioPanel};

const BAR_H: f32 = 32.0;

/// A real human bean
#[derive(Debug, GodotClass)]
#[class(base=PanelContainer)]
pub struct UiHomeTaskbar {
    selected_person: Option<Gd<Person>>,

    hbox: Gd<HBoxContainer>,

    ui_person_bio_panel: Gd<UiPersonBioPanel>,
    ui_person_needs_panel: Gd<UiPersonNeedsPanel>,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for UiHomeTaskbar {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            selected_person: None,

            hbox: HBoxContainer::new_alloc(),

            ui_person_bio_panel: UiPersonBioPanel::new_alloc(),
            ui_person_needs_panel: UiPersonNeedsPanel::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_ui();
    }

    fn process(&mut self, _delta: f64) {
        //
    }
}

impl UiHomeTaskbar {
    fn setup_ui(&mut self) {
        self.base_mut()
            .set_custom_minimum_size(Vector2::new(BAR_H, BAR_H));
        let stylebox: Gd<StyleBoxTexture> =
            load("res://assets/ui/theme_lunacy/stylebox_taskbar.tres");
        self.base_mut()
            .add_theme_stylebox_override("panel", &stylebox);
        self.base_mut()
            .set_anchors_preset(LayoutPreset::BOTTOM_LEFT);

        let mut hbox = self.hbox.clone();
        hbox.add_child(&self.ui_person_bio_panel);
        hbox.add_child(&self.ui_person_needs_panel);

        self.base_mut().add_child(&hbox);

        // Initialize with none state
        self.select_person(None);
    }

    pub fn select_person(&mut self, person: Option<Gd<Person>>) {
        self.ui_person_bio_panel
            .bind_mut()
            .select_person(person.clone());

        self.ui_person_needs_panel
            .bind_mut()
            .select_person(person.clone());

        if person.is_some() {
            self.ui_person_bio_panel.show();
            self.ui_person_needs_panel.show();
        } else {
            self.ui_person_bio_panel.hide();
            self.ui_person_needs_panel.hide();
        }

        self.selected_person = person;
    }
}
