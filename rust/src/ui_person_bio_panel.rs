//! Class: [UiPersonBioPanel]
//! Desc: A large panel that shows a person's info.
//!
use godot::{
    classes::{text_server::AutowrapMode, IMarginContainer, Label, MarginContainer, VBoxContainer},
    prelude::*,
};

use crate::{person::Task, Person};

const MIN_W: f32 = 128.0;

/// A real human bean
#[derive(Debug, GodotClass)]
#[class(base=MarginContainer)]
pub struct UiPersonBioPanel {
    selected_person: Option<Gd<Person>>,

    lab_person_name: Gd<Label>,
    lab_person_task: Gd<Label>,

    base: Base<MarginContainer>,
}

#[godot_api]
impl IMarginContainer for UiPersonBioPanel {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            selected_person: None,

            lab_person_name: Label::new_alloc(),
            lab_person_task: Label::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_ui();
    }

    fn process(&mut self, _delta: f64) {
        self.show_stats();
    }
}

impl UiPersonBioPanel {
    fn setup_ui(&mut self) {
        self.lab_person_name.set_name("lab_person_name");
        self.lab_person_name
            .set_autowrap_mode(AutowrapMode::ARBITRARY);

        self.lab_person_task.set_name("lab_person_task");
        self.lab_person_task
            .set_autowrap_mode(AutowrapMode::ARBITRARY);

        let mut vbox = VBoxContainer::new_alloc();
        vbox.add_child(&self.lab_person_name);
        vbox.add_child(&self.lab_person_task);

        self.base_mut().add_child(&vbox);
        self.base_mut()
            .set_custom_minimum_size(Vector2::new(MIN_W, 32.0));
    }

    pub fn select_person(&mut self, person: Option<Gd<Person>>) {
        self.selected_person = person;
    }

    fn show_stats(&mut self) {
        let Some(target) = self.selected_person.as_ref().map(|t| t.bind()) else {
            self.show_placeholder();
            return;
        };
        self.lab_person_name
            .set_text(&target.base().get_name().to_string());

        let task_text = match &target.task() {
            Task::Moving { queued_action, .. } => {
                format!("Preparing to\n'{}'", queued_action.key)
            }
            Task::Performing { action, .. } => action.to_present_tense(),
        };

        self.lab_person_task.set_text(task_text.as_str());
    }

    fn show_placeholder(&mut self) {
        self.lab_person_name.set_text("NONE!");
        self.lab_person_task.set_text("NONE!");
    }
}
