// SPDX-License-Identifier: LGPL-3.0-or-later
//! class: [Person]
//! desc: The human entity
//!
use std::collections::VecDeque;

use godot::prelude::*;
use uuid::Uuid;

use crate::{Action, EntityCollider, PersonAi, PersonNeeds, World};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Init,
    Moving,
    Waiting,
    InProgress,
    Done,
}

#[derive(Debug)]
pub struct Task {
    uuid: Uuid,
    state: TaskState,
    action: Action,
    target_position: Option<Vector3>,
    time_left: f64,
}

impl Task {
    pub fn new(action: Action) -> Self {
        let target_position = action.object.as_ref().map(|f| f.get_global_position());
        let uuid = action.master_uuid.unwrap_or(Uuid::new_v4());

        Self {
            uuid,
            state: TaskState::Init,
            action,
            target_position,
            time_left: 8.0,
        }
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn state(&self) -> TaskState {
        self.state
    }

    pub fn time_left(&self) -> f64 {
        self.time_left
    }
}

/// A real human bean
#[derive(Debug, GodotClass)]
#[class(no_init, base=Node3D)]
pub struct Person {
    uuid: Uuid,

    needs: PersonNeeds,
    brain: PersonAi,
    possible_actions: Vec<String>,
    task: Task,
    action_queue: VecDeque<Action>,

    world: Gd<World>,

    node_visuals: Gd<Node3D>,
    node_collider: Gd<EntityCollider>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Person {
    fn ready(&mut self) {
        self.setup_visuals();
        self.setup_collider();
    }

    fn process(&mut self, delta: f64) {
        self.needs.update(delta);

        let mut position = self.base_mut().get_position();
        let mut this_gd = self.to_gd();

        match self.task.state {
            TaskState::Init => {
                self.task.state = TaskState::Moving;
            }

            TaskState::Moving => {
                let Some(mut target_position) = self.task.target_position else {
                    self.task.state = TaskState::Waiting;
                    return;
                };

                if !self.task.action.is_primary() {
                    target_position += Vector3::RIGHT;
                }

                if position.distance_to(target_position) <= 0.5 {
                    this_gd.set_position(target_position);
                    self.task.state = TaskState::Waiting;
                    return;
                }

                let dir = (target_position - position).normalized();
                position += dir * 1.0 * delta as f32;
                self.node_visuals.look_at(target_position);
                self.base_mut().set_position(position);
            }

            TaskState::Waiting => match self.task.action.partner_uuid {
                Some(partner_uuid) => {
                    let world = self.world.bind();
                    if let Some(partner) = world.get_person(&partner_uuid) {
                        if partner.bind().task.action.master_uuid != Some(self.task.uuid) {
                            return;
                        }
                        match partner.bind().task.state {
                            TaskState::Init | TaskState::Moving => (),
                            TaskState::Waiting | TaskState::InProgress => {
                                self.task.state = TaskState::InProgress;

                                if self.task.action.key.as_str() == "do_the_mario" {
                                    let particles_packed: Gd<PackedScene> =
                                        load("res://assets/prefabs/vfx_particle_hearts.tscn");
                                    let particles = particles_packed.instantiate().unwrap();
                                    this_gd.add_child(&particles);
                                    this_gd.connect(
                                        "sig_task_ended",
                                        &particles.callable("queue_free"),
                                    );
                                }
                            }
                            TaskState::Done => panic!("wtf, partner ended task before I began?"),
                        }
                    }
                }
                None => {
                    if self.task.action.key.as_str() == "sleep" {
                        let particles_packed: Gd<PackedScene> =
                            load("res://assets/prefabs/vfx_particle_zzz.tscn");
                        let particles = particles_packed.instantiate().unwrap();
                        this_gd.add_child(&particles);
                        this_gd.connect("sig_task_ended", &particles.callable("queue_free"));
                    }

                    self.task.state = TaskState::InProgress;
                }
            },
            TaskState::InProgress => {
                self.task.time_left -= delta;

                match self.task.action.key.as_str() {
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
                    "idle" => {}
                    _ => panic!("unpossible!"),
                }

                if self.task.time_left <= 0.0 {
                    self.end_task();
                }
            }
            TaskState::Done => {
                // Clean up old task
                let old_action = &mut self.task.action;
                if let Some(old_target) = &mut old_action.object {
                    old_target.bind_mut().unreserve();
                }
                if let Some(partner_uuid) = old_action.partner_uuid {
                    let mut partner = self.world.bind().get_person(&partner_uuid).unwrap().clone();
                    partner.bind_mut().end_task_uuid(self.task.uuid);
                }

                self.start_new_task();
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
    pub fn new(world: Gd<World>) -> Gd<Self> {
        let uuid = Uuid::new_v4();

        Gd::from_init_fn(|base| Self {
            uuid,

            needs: PersonNeeds::default(),
            brain: PersonAi::new(uuid),
            possible_actions: vec![
                "make_food".into(),
                "toilet".into(),
                "sit".into(),
                "sleep".into(),
                //"do_the_mario".into(),
                "wash_hands".into(),
                "idle".into(),
            ],
            task: Task::new(Action::idle()),
            action_queue: VecDeque::new(),

            world,

            node_visuals: Node3D::new_alloc(),
            node_collider: EntityCollider::new_alloc(),

            base,
        })
    }

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

    fn end_task(&mut self) {
        self.task.state = TaskState::Done;
    }

    pub fn end_task_uuid(&mut self, uuid: Uuid) {
        if self.task.uuid == uuid {
            self.task.state = TaskState::Done;
        }
    }

    fn start_new_task(&mut self) {
        let this_gd = self.to_gd();

        self.base_mut().emit_signal("sig_task_ended", &[]);
        self.task = self.find_new_task();

        self.brain.last_action = self.task.action.key.clone();
        if let Some(target) = &mut self.task.action.object {
            target.bind_mut().reserve(this_gd.clone());
        }
    }

    fn find_new_task(&mut self) -> Task {
        let action = match self.action_queue.pop_front() {
            Some(action) => action,
            None => self.brain.decide_action(
                &self.needs,
                &self.world.bind().advertisements(),
                self.world.bind().people(),
                &self.possible_actions,
            ),
        };

        let task = Task::new(action);

        // Send a secondary copy to partner of group activity.
        if task.action.is_primary() {
            if let Some(partner_uuid) = task.action.partner_uuid {
                let mut secondary_action = task.action.clone();

                let mut partner = self.world.bind().get_person(&partner_uuid).unwrap().clone();
                secondary_action.partner_uuid = Some(self.uuid);
                secondary_action.master_uuid = Some(task.uuid());

                partner.bind_mut().queue_action(secondary_action);
            }
        }

        task
    }
}
