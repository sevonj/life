//! Class: [World]
//! Desc: World root node
//!
use std::{collections::HashMap, time::Duration};

use godot::prelude::*;

use godot::classes::{
    control::{LayoutPreset, MouseFilter, SizeFlags},
    node::ProcessMode,
    BoxShape3D, Control, Engine, HBoxContainer, InputEvent, InputEventMouseButton, MeshInstance3D,
    Shape3D, VBoxContainer,
};
use godot::global::MouseButton;
use uuid::Uuid;

use crate::{
    lot_builder::LotBuilder, lot_data, ActionAdvertisement, ActionAdvertisementStat,
    CameraRigOrbit, Furniture, Person, SpiritLevel, TimeScale, UiDebugOvl, UiWorldTaskbar,
    WorldEnv, WorldViewMode,
};

#[derive(Debug, GodotClass)]
#[class(base=Node)]
pub struct World {
    people: HashMap<Uuid, Gd<Person>>,
    furniture: Vec<Gd<Furniture>>,
    selected_person: Option<Gd<Person>>,
    view_mode: WorldViewMode,
    time_scale: TimeScale,
    time_of_day: Duration,

    lot_builder: Option<Gd<LotBuilder>>,

    ui_root: Gd<VBoxContainer>,
    ui_taskbar: Gd<UiWorldTaskbar>,
    spirit_level: Gd<SpiritLevel>,

    scn_root: Gd<Node3D>,
    scn_env: Gd<WorldEnv>,
    scn_camera_rig: Gd<CameraRigOrbit>,
    scn_lot_walls: Gd<MeshInstance3D>,

    data_walls: lot_data::Walls,

    base: Base<Node>,
}

#[godot_api]
impl INode for World {
    fn init(base: Base<Self::Base>) -> Self {
        let ui_taskbar = UiWorldTaskbar::new_alloc();
        let ui_root = VBoxContainer::new_alloc();

        let scn_root = Node3D::new_alloc();
        let scn_env = WorldEnv::new_alloc();
        let scn_camera_rig = CameraRigOrbit::new_alloc();
        let scn_lot_walls = MeshInstance3D::new_alloc();

        let data_walls = lot_data::Walls::with_test_layout();

        Self {
            people: HashMap::new(),
            furniture: vec![],
            selected_person: None,
            view_mode: WorldViewMode::default(),
            time_scale: TimeScale::Regular,
            time_of_day: Duration::from_secs(360 * 10),

            lot_builder: None,

            ui_root,
            ui_taskbar,
            spirit_level: SpiritLevel::new_alloc(),

            scn_root,
            scn_camera_rig,
            scn_env,
            scn_lot_walls,

            data_walls,

            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_process_mode(ProcessMode::ALWAYS);

        self.setup_ui();
        self.setup_scene();
        self.setup_objects();
        self.setup_people();

        self.base_mut().print_tree_pretty();
    }

    fn process(&mut self, delta: f64) {
        if !self.base().get_tree().unwrap().is_paused() {
            // 1 sec realtime => 1 min game time
            self.time_of_day += Duration::from_secs_f64(delta * 60.0);
        }
        const DAY_DURATION: Duration = Duration::from_secs(60 * 60 * 24);
        if self.time_of_day >= DAY_DURATION {
            self.time_of_day -= DAY_DURATION;
        }
        self.scn_env.bind_mut().set_time(self.time_of_day);

        let input = Input::singleton();

        if input.is_action_just_pressed("mode_play") {
            self.set_view_mode(WorldViewMode::Play);
            self.scn_lot_walls.show();
        } else if input.is_action_just_pressed("mode_buy") {
            self.set_view_mode(WorldViewMode::Buy);
            self.scn_lot_walls.show();
        } else if input.is_action_just_pressed("mode_build") {
            self.set_view_mode(WorldViewMode::Build);
            self.scn_lot_walls.hide();
        }

        match self.view_mode {
            WorldViewMode::Play => {
                if input.is_action_just_pressed("play_toggle_pause") {
                    let paused = self.base().get_tree().unwrap().is_paused();
                    self.base_mut().get_tree().unwrap().set_pause(!paused);
                } else if input.is_action_just_pressed("play_set_speed_1") {
                    self.set_time_scale(TimeScale::Regular);
                } else if input.is_action_just_pressed("play_set_speed_2") {
                    self.set_time_scale(TimeScale::Fast);
                } else if input.is_action_just_pressed("play_set_speed_3") {
                    self.set_time_scale(TimeScale::Superfast);
                } else if input.is_action_just_pressed("play_cycle_characters") {
                    self.select_next_person();
                }
            }
            WorldViewMode::Buy => (),
            WorldViewMode::Build => (),
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.clone().try_cast::<InputEventMouseButton>() {
            match event.get_button_index() {
                MouseButton::LEFT => {
                    if event.is_pressed() {
                        self.select_person(None);
                    }
                }
                _ => return,
            }
        }
    }
}

#[godot_api]
impl World {
    #[func]
    fn get_view_mode(&self) -> WorldViewMode {
        self.view_mode
    }

    #[func]
    pub fn set_view_mode(&mut self, mode: WorldViewMode) {
        if mode == self.view_mode {
            return;
        }

        if mode != WorldViewMode::Build {
            if let Some(lot_builder) = &mut self.lot_builder {
                self.data_walls = lot_builder.bind().wall_data().clone();
                lot_builder.queue_free();
                self.lot_builder = None;

                self.rebuild_building_mesh();
            }
        }

        match mode {
            WorldViewMode::Build => {
                self.base_mut().get_tree().unwrap().set_pause(true);

                if let Some(lot_builder) = &mut self.lot_builder {
                    godot_error!("wtf, lot builder exists already!");
                    lot_builder.queue_free();
                }
                let mut lot_builder =
                    LotBuilder::new(self.data_walls.clone(), self.scn_lot_walls.clone());
                lot_builder.set_name("lot_builder");

                self.base_mut().add_child(&lot_builder);
                self.lot_builder = Some(lot_builder);
            }
            WorldViewMode::Buy => {
                self.base_mut().get_tree().unwrap().set_pause(true);
            }
            WorldViewMode::Play => {
                self.base_mut().get_tree().unwrap().set_pause(false);
            }
        }
        self.view_mode = mode;
    }

    #[func]
    fn on_person_selected(&mut self, person: Gd<Person>) {
        self.select_person(Some(person));
    }
}

impl World {
    pub fn advertisements(&self) -> Vec<ActionAdvertisement> {
        let mut vec: Vec<ActionAdvertisement> = vec![];
        for furniture in &self.furniture {
            vec.extend(furniture.bind().available_actions().to_owned());
        }
        vec
    }

    pub fn get_person(&self, uuid: &Uuid) -> Option<&Gd<Person>> {
        self.people.get(uuid)
    }

    pub fn people(&self) -> &HashMap<Uuid, Gd<Person>> {
        &self.people
    }

    pub fn view_mode(&self) -> WorldViewMode {
        self.view_mode
    }

    fn setup_ui(&mut self) {
        // Empty space above taskbar
        let mut spacer = Control::new_alloc();
        spacer.set_v_size_flags(SizeFlags::EXPAND_FILL);
        spacer.set_mouse_filter(MouseFilter::IGNORE);
        spacer.set_name("spacer");

        self.ui_taskbar
            .set_anchors_preset(LayoutPreset::BOTTOM_LEFT);
        self.ui_taskbar.set_name("ui_taskbar");

        let mut ui_root = self.ui_root.clone();
        ui_root.set_anchors_preset(LayoutPreset::FULL_RECT);
        ui_root.add_child(&spacer);
        ui_root.add_child(&self.ui_taskbar);
        ui_root.set_process_mode(ProcessMode::ALWAYS);
        ui_root.set_name("ui_root");

        let mut ui_modehelp = UiDebugOvl::new_alloc();
        ui_modehelp.bind_mut().set_title("Modes");
        ui_modehelp.bind_mut().add_key("F1".into(), "Play mode");
        ui_modehelp.bind_mut().add_key("F2".into(), "Buy mode");
        ui_modehelp.bind_mut().add_key("F3".into(), "Build mode");
        ui_modehelp.set_name("ui_debug_ovl");

        let mut ui_camhelp = UiDebugOvl::new_alloc();
        ui_camhelp.bind_mut().set_title("Camera controls");
        ui_camhelp
            .bind_mut()
            .add_key("MMB | ctlr+RMB".into(), "Rotate");
        ui_camhelp
            .bind_mut()
            .add_key("WASD | arrows".into(), "Move");
        ui_camhelp.bind_mut().add_key("scroll | Z/X".into(), "Zoom");
        ui_camhelp.set_name("ui_controls_cam");

        let mut ui_playhelp = UiDebugOvl::new_alloc();
        ui_playhelp.bind_mut().set_title("Play mode");
        ui_playhelp
            .bind_mut()
            .add_key("LMB".into(), "Select character");
        ui_playhelp
            .bind_mut()
            .add_key("TAB".into(), "Cycle characters");
        ui_playhelp
            .bind_mut()
            .add_key("1/2/3".into(), "Regular/Fast/Superfast times");
        ui_playhelp
            .bind_mut()
            .add_key("space".into(), "Toggle pause");
        ui_playhelp.set_name("ui_controls_play");

        let mut ui_buildhelp = UiDebugOvl::new_alloc();
        ui_buildhelp.bind_mut().set_title("Build mode");
        ui_buildhelp.bind_mut().add_key("LMB".into(), "Build wall");
        ui_buildhelp
            .bind_mut()
            .add_key("ctrl+LMB".into(), "Remove wall");
        ui_buildhelp.set_name("ui_controls_cam");

        let mut ui_debug_root = HBoxContainer::new_alloc();
        ui_debug_root.add_child(&ui_modehelp);
        ui_debug_root.add_child(&ui_camhelp);
        ui_debug_root.add_child(&ui_playhelp);
        ui_debug_root.add_child(&ui_buildhelp);
        ui_debug_root.set_process_mode(ProcessMode::ALWAYS);
        ui_debug_root.set_name("ui_debug_root");

        self.base_mut().add_child(&ui_root);
        self.base_mut().add_child(&ui_debug_root);

        // Initialize with none state
        self.select_person(None);
    }

    fn setup_scene(&mut self) {
        self.scn_camera_rig.set_name("scn_camera_rig");

        self.scn_env.set_name("scn_env");

        let terrain_packed: Gd<PackedScene> = load("res://assets/models/mdl_lot_32x32.blend");
        let mut terrain = terrain_packed.instantiate().unwrap();
        terrain.set_name("terrain");

        let mut scn_lot_walls = self.scn_lot_walls.clone();
        scn_lot_walls.set_name("lot_walls");

        let mut spirit_level = self.spirit_level.clone();
        spirit_level.set_name("spirit_level");

        let mut scn_root = self.scn_root.clone();
        scn_root.add_child(&self.scn_camera_rig);
        scn_root.add_child(&self.scn_env);
        scn_root.add_child(&terrain);
        scn_root.add_child(&scn_lot_walls);
        scn_root.add_child(&spirit_level);
        scn_root.set_process_mode(ProcessMode::PAUSABLE);
        scn_root.set_name("scn_root");

        self.base_mut().add_child(&scn_root);

        self.rebuild_building_mesh();
    }

    fn setup_objects(&mut self) {
        let mut stove_coll_box = BoxShape3D::new_gd();
        stove_coll_box.set_size(Vector3::new(1.0, 1.0, 1.0));
        let stove_coll_offset = stove_coll_box.get_size() / 2.0;
        let stove_coll_shape = stove_coll_box.upcast::<Shape3D>();

        let mut stove = Furniture::new(
            "res://assets/furniture/mdl_appliance_stove_1x1_001.blend",
            stove_coll_shape,
            stove_coll_offset,
            vec![],
        );
        let stove_ref = stove.clone();
        stove
            .bind_mut()
            .available_actions_mut()
            .push(ActionAdvertisement {
                action_key: "make_food".into(),
                source_node: stove_ref,
                stats: vec![
                    ActionAdvertisementStat {
                        key: "hunger".into(),
                        value: 10,
                    },
                    ActionAdvertisementStat {
                        key: "bladder".into(),
                        value: -2,
                    },
                ],
                required_people: 1,
            });

        let mut bed_coll_box = BoxShape3D::new_gd();
        bed_coll_box.set_size(Vector3::new(0.9, 0.6, 2.0));
        let bed_coll_offset = bed_coll_box.get_size() / 2.0;
        let bed_coll_shape = bed_coll_box.upcast::<Shape3D>();

        let mut bed = Furniture::new(
            "res://assets/furniture/mdl_bed_1x2_001.blend",
            bed_coll_shape,
            bed_coll_offset,
            vec![],
        );
        let bed_ref = bed.clone();
        bed.bind_mut()
            .available_actions_mut()
            .push(ActionAdvertisement {
                action_key: "sleep".into(),
                source_node: bed_ref,
                stats: vec![ActionAdvertisementStat {
                    key: "sleep".into(),
                    value: 10,
                }],
                required_people: 1,
            });

        let mut bed2_coll_box = BoxShape3D::new_gd();
        bed2_coll_box.set_size(Vector3::new(1.8, 0.6, 2.0));
        let bed2_coll_offset = bed2_coll_box.get_size() / 2.0;
        let bed2_coll_shape = bed2_coll_box.upcast::<Shape3D>();

        let mut bed2 = Furniture::new(
            "res://assets/furniture/mdl_bed_double_2x2_001.blend",
            bed2_coll_shape,
            bed2_coll_offset,
            vec![],
        );
        let bed_ref = bed2.clone();
        bed2.bind_mut()
            .available_actions_mut()
            .push(ActionAdvertisement {
                action_key: "sleep".into(),
                source_node: bed_ref,
                stats: vec![ActionAdvertisementStat {
                    key: "sleep".into(),
                    value: 10,
                }],
                required_people: 1,
            });
        let bed_ref = bed2.clone();
        bed2.bind_mut()
            .available_actions_mut()
            .push(ActionAdvertisement {
                action_key: "do_the_mario".into(),
                source_node: bed_ref,
                stats: vec![
                    ActionAdvertisementStat {
                        key: "social".into(),
                        value: 4,
                    },
                    ActionAdvertisementStat {
                        key: "fun".into(),
                        value: 4,
                    },
                ],
                required_people: 2,
            });

        let mut toilet_coll_box = BoxShape3D::new_gd();
        toilet_coll_box.set_size(Vector3::new(1.0, 1.0, 1.0));
        let toilet_coll_offset = toilet_coll_box.get_size() / 2.0;
        let mut toilet = Furniture::new(
            "res://assets/furniture/mdl_toilet_1x1_001.blend",
            toilet_coll_box.upcast::<Shape3D>(),
            toilet_coll_offset,
            vec![],
        );
        let toilet_ref = toilet.clone();
        toilet
            .bind_mut()
            .available_actions_mut()
            .push(ActionAdvertisement {
                action_key: "sit".into(),
                source_node: toilet_ref,
                stats: vec![ActionAdvertisementStat {
                    key: "comfort".into(),
                    value: 3,
                }],
                required_people: 1,
            });
        let toilet_ref = toilet.clone();
        toilet
            .bind_mut()
            .available_actions_mut()
            .push(ActionAdvertisement {
                action_key: "toilet".into(),
                source_node: toilet_ref,
                stats: vec![ActionAdvertisementStat {
                    key: "bladder".into(),
                    value: 10,
                }],
                required_people: 1,
            });

        let mut sink_coll_box = BoxShape3D::new_gd();
        sink_coll_box.set_size(Vector3::new(1.0, 1.0, 1.0));
        let sink_coll_offset = sink_coll_box.get_size() / 2.0;
        let mut sink = Furniture::new(
            "res://assets/furniture/mdl_sink_1x1_001.blend",
            sink_coll_box.upcast::<Shape3D>(),
            sink_coll_offset,
            vec![],
        );
        let sink_ref = sink.clone();
        sink.bind_mut()
            .available_actions_mut()
            .push(ActionAdvertisement {
                action_key: "wash_hands".into(),
                source_node: sink_ref,
                stats: vec![ActionAdvertisementStat {
                    key: "hygiene".into(),
                    value: 10,
                }],
                required_people: 1,
            });

        stove.set_position(Vector3::new(8.0, 0.0, 15.0));
        bed.set_position(Vector3::new(11.0, 0.0, 15.0));
        bed2.set_position(Vector3::new(11.0, 0.0, 20.0));
        toilet.set_position(Vector3::new(15.0, 0.0, 15.0));
        sink.set_position(Vector3::new(14.0, 0.0, 15.0));
        self.add_furniture(stove);
        self.add_furniture(bed);
        self.add_furniture(bed2);
        self.add_furniture(toilet);
        self.add_furniture(sink);
    }

    fn setup_people(&mut self) {
        let mut alice = Person::new(self.to_gd());
        alice.set_position(Vector3::new(5.0, 0.0, 10.0));
        alice.set_name("alice");

        let mut bob = Person::new(self.to_gd());
        bob.set_position(Vector3::new(7.0, 0.0, 7.0));
        bob.set_name("bob");

        self.add_person(alice);
        self.add_person(bob);
    }

    pub fn add_person(&mut self, mut person: Gd<Person>) {
        person.connect("sig_selected", &self.to_gd().callable("on_person_selected"));
        let uuid = person.bind().uuid();

        self.scn_root.add_child(&person);
        self.people.insert(uuid, person);
    }

    pub fn add_furniture(&mut self, furniture: Gd<Furniture>) {
        //furniture.connect("sig_selected", &self.to_gd().callable("on_person_selected"));

        self.scn_root.add_child(&furniture);
        self.furniture.push(furniture);
    }

    pub fn select_person(&mut self, person: Option<Gd<Person>>) {
        self.selected_person = person.clone();
        self.ui_taskbar.bind_mut().select_person(person);
        self.spirit_level.bind_mut().target = self.selected_person.clone();
    }

    pub fn select_next_person(&mut self) {
        let Some(current) = self.selected_person.as_ref().map(|p| p.bind().uuid()) else {
            for person in self.people.values() {
                self.select_person(Some(person.clone()));
                return;
            }
            return;
        };
        let mut prev = Uuid::nil();
        for person in self.people.values() {
            if prev == current {
                self.select_person(Some(person.clone()));
                return;
            }
            prev = person.bind().uuid()
        }
        for person in self.people.values() {
            if prev == current {
                self.select_person(Some(person.clone()));
                return;
            }
            prev = person.bind().uuid()
        }
    }

    fn rebuild_building_mesh(&mut self) {
        let mesh = self.data_walls.to_mesh();
        self.scn_lot_walls.set_mesh(&mesh);
    }

    fn set_time_scale(&mut self, time_scale: TimeScale) {
        self.base_mut().get_tree().unwrap().set_pause(false);
        Engine::singleton().set_time_scale(time_scale.to_engine_time());

        self.time_scale = time_scale
    }
}
