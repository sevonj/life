//! Class: [UiPersonNeedsPanel]
//! Desc: A large panel that shows a person's needs.
//!
use godot::{
    classes::{
        GridContainer, IMarginContainer, Label, MarginContainer, ProgressBar, VBoxContainer,
    },
    prelude::*,
};

use crate::Person;

const BAR_W: f32 = 128.0;
const BAR_H: f32 = 8.0;

/// A real human bean
#[derive(Debug, GodotClass)]
#[class(base=MarginContainer)]
pub struct UiPersonNeedsPanel {
    selected_person: Option<Gd<Person>>,
    bar_bladder: Gd<ProgressBar>,
    bar_comfort: Gd<ProgressBar>,
    bar_environment: Gd<ProgressBar>,
    bar_fun: Gd<ProgressBar>,
    bar_hunger: Gd<ProgressBar>,
    bar_hygiene: Gd<ProgressBar>,
    bar_sleep: Gd<ProgressBar>,
    bar_social: Gd<ProgressBar>,

    base: Base<MarginContainer>,
}

#[godot_api]
impl IMarginContainer for UiPersonNeedsPanel {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            selected_person: None,
            bar_bladder: ProgressBar::new_alloc(),
            bar_comfort: ProgressBar::new_alloc(),
            bar_environment: ProgressBar::new_alloc(),
            bar_fun: ProgressBar::new_alloc(),
            bar_hunger: ProgressBar::new_alloc(),
            bar_hygiene: ProgressBar::new_alloc(),
            bar_sleep: ProgressBar::new_alloc(),
            bar_social: ProgressBar::new_alloc(),

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

impl UiPersonNeedsPanel {
    fn setup_ui(&mut self) {
        fn build_stat(bar: &mut Gd<ProgressBar>, title: &str) -> Gd<VBoxContainer> {
            bar.set_max(1.0);
            bar.set_show_percentage(false);
            bar.set_custom_minimum_size(Vector2::new(BAR_W, BAR_H));
            bar.set_name("bar");

            let mut label = Label::new_alloc();
            label.set_name("title");
            label.set_text(title);

            let mut vbox = VBoxContainer::new_alloc();
            vbox.add_child(&label);
            vbox.add_child(&*bar);
            vbox
        }

        let mut grid = GridContainer::new_alloc();
        grid.set_columns(2);

        // Row 1
        grid.add_child(&build_stat(&mut self.bar_hunger, "Hunger"));
        grid.add_child(&build_stat(&mut self.bar_sleep, "Sleep"));
        // Row 2
        grid.add_child(&build_stat(&mut self.bar_comfort, "Comfort"));
        grid.add_child(&build_stat(&mut self.bar_fun, "Fun"));
        // Row 3
        grid.add_child(&build_stat(&mut self.bar_hygiene, "Hygiene"));
        grid.add_child(&build_stat(&mut self.bar_social, "Social"));
        // Row 4
        grid.add_child(&build_stat(&mut self.bar_bladder, "Bladder"));
        grid.add_child(&build_stat(&mut self.bar_environment, "Environment"));

        self.base_mut().add_child(&grid);
    }

    pub fn select_person(&mut self, person: Option<Gd<Person>>) {
        self.selected_person = person;
    }

    fn show_stats(&mut self) {
        let Some(target) = self.selected_person.as_ref().map(|t| t.bind()) else {
            self.show_placeholder();
            return;
        };
        let needs = target.needs();
        self.bar_bladder.set_value(needs.bladder());
        self.bar_comfort.set_value(needs.comfort());
        self.bar_environment.set_value(needs.environment());
        self.bar_fun.set_value(needs.fun());
        self.bar_hunger.set_value(needs.hunger());
        self.bar_hygiene.set_value(needs.hygiene());
        self.bar_sleep.set_value(needs.sleep());
        self.bar_social.set_value(needs.social());
    }

    fn show_placeholder(&mut self) {
        self.bar_bladder.set_value(0.0);
        self.bar_comfort.set_value(0.0);
        self.bar_environment.set_value(0.0);
        self.bar_fun.set_value(0.0);
        self.bar_hunger.set_value(0.0);
        self.bar_hygiene.set_value(0.0);
        self.bar_sleep.set_value(0.0);
        self.bar_social.set_value(0.0);
    }
}
