//! class: [Person]
//! desc: The human entity
//!
use godot::prelude::*;

use crate::{Action, EntityCollider, PersonAi, PersonNeeds, World};

#[derive(Debug)]
pub enum Task {
    /// Moving to the next action
    Moving {
        queued_action: Action,
        target_position: Vector3,
    },
    /// Performing an action. Time left is in seconds.
    Performing { action: Action, time_left: f64 },
}

/// A real human bean
#[derive(Debug, GodotClass)]
#[class(base=Node3D)]
pub struct Person {
    needs: PersonNeeds,
    brain: PersonAi,
    possible_actions: Vec<String>,
    task: Task,

    pub world: Option<Gd<World>>,

    node_visuals: Gd<Node3D>,
    node_collider: Gd<EntityCollider>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Person {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            needs: PersonNeeds::default(),
            brain: PersonAi::default(),
            possible_actions: vec![
                "make_food".into(),
                "toilet".into(),
                "sit".into(),
                "sleep".into(),
                "do_the_mario".into(),
                "wash_hands".into(),
                "idle".into(),
            ],
            task: Task::Performing {
                action: Action::idle(),
                time_left: 1.0,
            },

            world: None,

            node_visuals: Node3D::new_alloc(),
            node_collider: EntityCollider::new_alloc(),

            base,
        }
    }

    fn ready(&mut self) {
        self.setup_visuals();
        self.setup_collider();
    }

    fn process(&mut self, delta: f64) {
        self.needs.update(delta);

        let mut position = self.base_mut().get_position();
        let mut this_gd = self.to_gd();

        match &mut self.task {
            Task::Moving {
                queued_action,
                target_position,
            } => {
                let target_position = target_position.to_owned();

                if position.distance_to(target_position) <= 0.5 {
                    this_gd.set_position(target_position);

                    match queued_action.key.as_str() {
                        "sleep" => {
                            let sleep_particles_packed: Gd<PackedScene> =
                                load("res://assets/prefabs/vfx_particle_zzz.tscn");
                            let sleep_particles = sleep_particles_packed.instantiate().unwrap();
                            this_gd.add_child(&sleep_particles);
                            this_gd
                                .connect("sig_task_ended", &sleep_particles.callable("queue_free"));
                        }
                        "do_the_mario" => {
                            let love_particles_packed: Gd<PackedScene> =
                                load("res://assets/prefabs/vfx_particle_hearts.tscn");
                            let love_particles = love_particles_packed.instantiate().unwrap();
                            this_gd.add_child(&love_particles);
                            this_gd
                                .connect("sig_task_ended", &love_particles.callable("queue_free"));
                        }
                        _ => (),
                    }

                    self.task = Task::Performing {
                        action: queued_action.to_owned(),
                        time_left: 10.0,
                    };

                    return;
                }

                let dir = (target_position - position).normalized();
                position += dir * 1.0 * delta as f32;
                self.node_visuals.look_at(target_position);
                self.base_mut().set_position(position);
            }
            Task::Performing { action, time_left } => {
                *time_left -= delta;

                match action.key.as_str() {
                    "make_food" => {
                        self.needs.hunger += 0.2 * delta;
                    }
                    "toilet" => {
                        self.needs.bladder += 0.2 * delta;
                    }
                    "sit" => {
                        self.needs.comfort += 0.2 * delta;
                    }
                    "sleep" => {
                        self.needs.sleep += 0.2 * delta;
                        self.needs.comfort += 0.2 * delta;
                    }
                    "do_the_mario" => {
                        self.needs.fun += 0.2 * delta;
                        self.needs.social += 0.2 * delta;
                        self.needs.comfort += 0.2 * delta;
                        self.needs.hygiene -= 0.1 * delta;
                    }
                    "wash_hands" => {
                        self.needs.hygiene += 0.2 * delta;
                    }
                    "idle" => (),
                    _ => panic!("unpossible!"),
                }

                if *time_left <= 0.0 {
                    // Clean up old action
                    let old_action = match &mut self.task {
                        Task::Moving { queued_action, .. } => queued_action,
                        Task::Performing { action, .. } => action,
                    };
                    if let Some(old_target) = &mut old_action.target {
                        old_target.bind_mut().unreserve();
                    }

                    // Switch task
                    self.base_mut().emit_signal("sig_task_ended", &[]);
                    self.task = self.find_new_task();

                    // Setup new action
                    let action = match &mut self.task {
                        Task::Moving { queued_action, .. } => queued_action,
                        Task::Performing { action, .. } => action,
                    };
                    self.brain.last_action = action.key.clone();
                    if let Some(target) = &mut action.target {
                        target.bind_mut().reserve(this_gd.clone());
                    }
                }
            }
        }
    }
}

#[godot_api]
impl Person {
    /// This person was selected.
    #[signal]
    fn sig_selected(person: Gd<Person>);

    #[signal]
    fn sig_task_ended();

    /// When [node_collider] is clicked.
    #[func]
    fn on_click(&mut self) {
        let this = self.to_gd().to_variant();
        self.base_mut().emit_signal("sig_selected", &[this]);
    }
}

impl Person {
    pub fn needs(&self) -> &PersonNeeds {
        &self.needs
    }

    pub fn needs_mut(&mut self) -> &mut PersonNeeds {
        &mut self.needs
    }

    pub fn task(&self) -> &Task {
        &self.task
    }

    fn setup_visuals(&mut self) {
        let person_packed: Gd<PackedScene> = load("res://assets/characters/mdl_person_base.blend");
        let mut person_model = person_packed.instantiate().unwrap();
        person_model.set_name("person_model");

        let mut node_visuals = self.node_visuals.clone();
        node_visuals.add_child(&person_model);
        node_visuals.set_name("visuals");

        self.base_mut().add_child(&node_visuals);
    }

    fn setup_collider(&mut self) {
        let mut node_collider = self.node_collider.clone();
        node_collider.connect("sig_clicked", &self.to_gd().callable("on_click"));
        node_collider.set_name("collider");

        self.base_mut().add_child(&node_collider);
    }

    fn find_new_task(&self) -> Task {
        let Some(world) = &self.world else {
            panic!("No world!");
        };

        let world = world.clone();

        let action = self.brain.decide_action(
            &self.needs,
            &world.bind().advertisements(),
            &self.possible_actions,
        );

        let target_position = match &action.target {
            Some(target) => target.get_position(),
            None => self.base().get_position(),
        };

        Task::Moving {
            queued_action: action,
            target_position,
        }
    }
}
