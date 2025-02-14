//! Class: [UiHomeTaskbar]
//! Desc: The large bottom bar in home view
//!
use godot::classes::{
    control::LayoutPreset, Control, HBoxContainer, IPanelContainer, PanelContainer, Script, StyleBoxTexture
};
use godot::prelude::*;

use crate::{Person, UiPersonBioPanel, UiPersonNeedsPanel};

const BAR_H: f32 = 32.0;

/// A real human bean
#[derive(Debug, GodotClass)]
#[class(base=PanelContainer)]
pub struct UiWorldTaskbar {
    selected_person: Option<Gd<Person>>,

    hbox: Gd<HBoxContainer>,

    ui_world_mode_select: Gd<Control>,

    ui_playmode_root: Gd<HBoxContainer>,
    ui_playmode_person_bio: Gd<UiPersonBioPanel>,
    ui_playmode_person_needs: Gd<UiPersonNeedsPanel>,

    ui_buildmode_root: Gd<HBoxContainer>,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for UiWorldTaskbar {
    fn init(base: Base<Self::Base>) -> Self {
        //let ui_world_mode_select_packed: Gd<Script> = load("res://classes/ui_world_mode_select.gd");
        //let pp = ui_world_mode_select_packed.new_al

        Self {
            selected_person: None,

            hbox: HBoxContainer::new_alloc(),

            ui_world_mode_select: Control::new_alloc(),

            ui_playmode_root: HBoxContainer::new_alloc(),
            ui_playmode_person_bio: UiPersonBioPanel::new_alloc(),
            ui_playmode_person_needs: UiPersonNeedsPanel::new_alloc(),

            ui_buildmode_root: HBoxContainer::new_alloc(),

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

impl UiWorldTaskbar {
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
        hbox.add_child(&self.ui_world_mode_select);
        hbox.add_child(&self.ui_playmode_person_bio);
        hbox.add_child(&self.ui_playmode_person_needs);

        self.base_mut().add_child(&hbox);

        // Initialize with none state
        self.select_person(None);
    }

    // /// Signals
    //pub fn connect_world(&mut self, world: Gd<World>) {
    //    //self.ui_world_mode_select.bind_mut().connect_world(world)
    //}

    pub fn select_person(&mut self, person: Option<Gd<Person>>) {
        self.ui_playmode_person_bio
            .bind_mut()
            .select_person(person.clone());

        self.ui_playmode_person_needs
            .bind_mut()
            .select_person(person.clone());

        if person.is_some() {
            self.ui_playmode_person_bio.show();
            self.ui_playmode_person_needs.show();
        } else {
            self.ui_playmode_person_bio.hide();
            self.ui_playmode_person_needs.hide();
        }

        self.selected_person = person;
    }
}
