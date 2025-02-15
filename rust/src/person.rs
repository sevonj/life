//! class: [Person]
//! desc: The human entity
//!
use std::collections::VecDeque;

use godot::prelude::*;
use uuid::Uuid;

use crate::{action, Action, EntityCollider, PersonAi, PersonNeeds, World};

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
    uuid: Uuid,

    needs: PersonNeeds,
    brain: PersonAi,
    possible_actions: Vec<String>,
    task: Task,
    task_done: bool,
    action_queue: VecDeque<Action>,

    pub world: Option<Gd<World>>,

    node_visuals: Gd<Node3D>,
    node_collider: Gd<EntityCollider>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Person {
    fn init(base: Base<Self::Base>) -> Self {
        let uuid = Uuid::new_v4();

        Self {
            uuid,

            needs: PersonNeeds::default(),
            brain: PersonAi::new(uuid),
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
            task_done: false,
            action_queue: VecDeque::new(),

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

        if self.task_done{
            self.start_new_task();
        }

        match &mut self.task {
            Task::Moving {
                queued_action,
                target_position,
            } => {
                let real_target_position = match queued_action.primary {
                    true => *target_position,
                    false => *target_position + Vector3::RIGHT,
                };

                if position.distance_to(real_target_position) <= 0.5 {
                    this_gd.set_position(real_target_position);

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
                            let Some(partner_uuid) = &queued_action.partner_uuid else {
                                godot_error!("you can't do the mario without the luigi!");
                                self.end_task();
                                return;
                            };
                            let world = self.world.as_ref().unwrap().bind();
                            if let Some(partner) = world.people().get(&partner_uuid).clone() {
                                match partner.bind().task() {
                                    Task::Moving { queued_action, .. } => {
                                        if queued_action.key != "do_the_mario" {
                                            return;
                                        }
                                        if queued_action.partner_uuid != Some(self.uuid) {
                                            return;
                                        };
                                    }
                                    Task::Performing { action, .. } => {
                                        if action.key == "do_the_mario" {
                                            let love_particles_packed: Gd<PackedScene> = load(
                                                "res://assets/prefabs/vfx_particle_hearts.tscn",
                                            );
                                            let love_particles =
                                                love_particles_packed.instantiate().unwrap();
                                            this_gd.add_child(&love_particles);
                                            this_gd.connect(
                                                "sig_task_ended",
                                                &love_particles.callable("queue_free"),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        _ => (),
                    }

                    self.task = Task::Performing {
                        action: queued_action.to_owned(),
                        time_left: 10.0,
                    };

                    return;
                }

                let dir = (real_target_position - position).normalized();
                position += dir * 1.0 * delta as f32;
                self.node_visuals.look_at(real_target_position);
                self.base_mut().set_position(position);
            }
            Task::Performing { action, time_left } => {
                match action.key.as_str() {
                    "make_food" => {
                        *time_left -= delta;
                        self.needs.hunger += 0.2 * delta;
                    }
                    "toilet" => {
                        *time_left -= delta;
                        self.needs.bladder += 0.2 * delta;
                    }
                    "sit" => {
                        *time_left -= delta;
                        self.needs.comfort += 0.2 * delta;
                    }
                    "sleep" => {
                        *time_left -= delta;
                        self.needs.sleep += 0.2 * delta;
                        self.needs.comfort += 0.2 * delta;
                    }
                    "do_the_mario" => {
                        let Some(partner_uuid) = &action.partner_uuid else {
                            godot_error!("you can't do the mario without the luigi!");
                            self.end_task();
                            return;
                        };
                        let world = self.world.as_ref().unwrap().bind();
                        let partner = world.people().get(partner_uuid).unwrap();
                        match partner.bind().task() {
                            Task::Performing { action, .. } => {
                                if action.key != "do_the_mario" {
                                    return;
                                }
                                if action.partner_uuid != Some(self.uuid) {
                                    return;
                                };
                            }
                            _ => return,
                        }
                        *time_left -= delta;
                        self.needs.fun += 0.2 * delta;
                        self.needs.social += 0.2 * delta;
                        self.needs.comfort += 0.2 * delta;
                        self.needs.hygiene -= 0.1 * delta;
                    }
                    "wash_hands" => {
                        *time_left -= delta;
                        self.needs.hygiene += 0.2 * delta;
                    }
                    "idle" => {
                        *time_left -= delta;
                    }
                    _ => panic!("unpossible!"),
                }

                if *time_left <= 0.0 {
                    // Clean up old action
                    let old_action = match &mut self.task {
                        Task::Moving { queued_action, .. } => queued_action,
                        Task::Performing { action, .. } => action,
                    };
                    if let Some(old_target) = &mut old_action.object {
                        old_target.bind_mut().unreserve();
                    }
                    if let Some(partner_uuid) = old_action.partner_uuid {
                        let world = self.world.as_ref().unwrap().bind();
                        let mut partner = world.people().get(&partner_uuid).unwrap().clone();
                        partner.bind_mut().end_task();
                    }

                    self.end_task();
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
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

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

    pub fn queue_action(&mut self, action: Action) {
        self.action_queue.push_back(action);
    }

    pub fn end_task(&mut self) {
        self.task_done = true;
    }

    fn start_new_task(&mut self) {
        let this_gd = self.to_gd();

        self.base_mut().emit_signal("sig_task_ended", &[]);
        self.task = self.find_new_task();

        // Setup new action
        let action = match &mut self.task {
            Task::Moving { queued_action, .. } => queued_action,
            Task::Performing { action, .. } => action,
        };
        self.brain.last_action = action.key.clone();
        if let Some(target) = &mut action.object {
            target.bind_mut().reserve(this_gd.clone());
        }
        self.task_done = false;
    }

    fn find_new_task(&mut self) -> Task {
        let Some(world) = &self.world else {
            panic!("No world!");
        };

        let world = world.clone();

        let action = match self.action_queue.pop_front() {
            Some(action) => action,
            None => self.brain.decide_action(
                &self.needs,
                &world.bind().advertisements(),
                &world.bind().people(),
                &self.possible_actions,
            ),
        };

        let target_position = match &action.object {
            Some(target) => target.get_position(),
            None => self.base().get_position(),
        };

        if action.primary {
            if let Some(partner_uuid) = action.partner_uuid.clone() {
                let mut secondary_action = action.clone();

                let world = self.world.as_ref().unwrap().bind();
                let mut partner = world.people().get(&partner_uuid).unwrap().clone();
                secondary_action.partner_uuid = Some(self.uuid);
                secondary_action.primary = false;
                partner.bind_mut().queue_action(secondary_action);
            }
        }

        Task::Moving {
            queued_action: action,
            target_position,
        }
    }
}
