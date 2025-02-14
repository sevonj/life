//! Class: [World]
//! Desc: World root node
//!
use godot::classes::{
    control::{LayoutPreset, MouseFilter, SizeFlags},
    node::ProcessMode,
    BoxShape3D, Control, InputEvent, InputEventMouseButton, MeshInstance3D, Shape3D, VBoxContainer,
};
use godot::global::MouseButton;
use godot::prelude::*;

use crate::{
    lot_builder, ActionAdvertisement, ActionAdvertisementStat, CameraRigOrbit, Furniture,
    LotBuilder, LotWalls, Person, UiWorldTaskbar, WorldEnv, WorldViewMode,
};

#[derive(Debug, GodotClass)]
#[class(base=Node)]
pub struct World {
    people: Vec<Gd<Person>>,
    furniture: Vec<Gd<Furniture>>,
    selected_person: Option<Gd<Person>>,
    view_mode: WorldViewMode,

    lot_builder: Option<Gd<LotBuilder>>,

    ui_root: Gd<VBoxContainer>,
    ui_taskbar: Gd<UiWorldTaskbar>,

    scn_root: Gd<Node3D>,
    scn_env: Gd<WorldEnv>,
    scn_camera_rig: Gd<CameraRigOrbit>,
    scn_lot_walls: LotWalls,

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

        let scn_lot_walls = LotWalls::with_test_layout();

        Self {
            people: vec![],
            furniture: vec![],
            selected_person: None,
            view_mode: WorldViewMode::default(),

            lot_builder: None,

            ui_root,
            ui_taskbar,

            scn_root,
            scn_camera_rig,
            scn_env,
            scn_lot_walls,

            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_process_mode(ProcessMode::ALWAYS);

        //let lot_builder = self.lot_builder.clone().unwrap();
        //self.base_mut().add_child(&lot_builder);

        self.setup_ui();
        self.setup_scene();
        self.setup_objects();
        self.setup_people();

        self.base_mut().print_tree_pretty();
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        if input.is_action_just_pressed("mode_play") {
            self.set_view_mode(WorldViewMode::Play);
        } else if input.is_action_just_pressed("mode_buy") {
            self.set_view_mode(WorldViewMode::Buy);
        } else if input.is_action_just_pressed("mode_build") {
            self.set_view_mode(WorldViewMode::Build);
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
        match mode {
            WorldViewMode::Build => {
                self.base_mut().get_tree().unwrap().set_pause(true);

                if let Some(lot_builder) = &mut self.lot_builder {
                    godot_error!("wtf, lot builder exists already!");
                    lot_builder.queue_free();
                }
                let mut lot_builder = LotBuilder::new_alloc();
                lot_builder.set_name("lot_builder");

                self.base_mut().add_child(&lot_builder);
                self.lot_builder = Some(lot_builder);
            }
            WorldViewMode::Buy => {
                self.base_mut().get_tree().unwrap().set_pause(true);

                if let Some(lot_builder) = &mut self.lot_builder {
                    lot_builder.queue_free();
                    self.lot_builder = None;
                }
            }
            WorldViewMode::Play => {
                self.base_mut().get_tree().unwrap().set_pause(false);

                if let Some(lot_builder) = &mut self.lot_builder {
                    lot_builder.queue_free();
                    self.lot_builder = None;
                }
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
        let this_gd = self.to_gd();
        self.ui_taskbar.bind_mut().connect_world(this_gd);
        self.ui_taskbar.set_name("ui_taskbar");

        let mut ui_root = self.ui_root.clone();
        ui_root.set_anchors_preset(LayoutPreset::FULL_RECT);
        ui_root.add_child(&spacer);
        ui_root.add_child(&self.ui_taskbar);
        ui_root.set_process_mode(ProcessMode::ALWAYS);
        ui_root.set_name("ui_root");

        self.base_mut().add_child(&ui_root);

        // Initialize with none state
        self.select_person(None);
    }

    fn setup_scene(&mut self) {
        self.scn_camera_rig.set_name("scn_camera_rig");

        self.scn_env.set_name("scn_env");

        let terrain_packed: Gd<PackedScene> = load("res://assets/models/mdl_lot_32x32.blend");
        let mut terrain = terrain_packed.instantiate().unwrap();
        terrain.set_name("terrain");

        let mut lot_walls = MeshInstance3D::new_alloc();
        lot_walls.set_mesh(&self.scn_lot_walls.generate_mesh());
        lot_walls.set_name("lot_walls");

        let mut scn_root = self.scn_root.clone();
        scn_root.add_child(&self.scn_camera_rig);
        scn_root.add_child(&self.scn_env);
        scn_root.add_child(&terrain);
        scn_root.add_child(&lot_walls);
        scn_root.set_process_mode(ProcessMode::PAUSABLE);
        scn_root.set_name("scn_root");

        self.base_mut().add_child(&scn_root);
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
            });

        let mut bed2_coll_box = BoxShape3D::new_gd();
        bed2_coll_box.set_size(Vector3::new(0.9, 0.6, 2.0));
        let bed2_coll_offset = bed2_coll_box.get_size() / 2.0;
        let bed2_coll_shape = bed2_coll_box.upcast::<Shape3D>();

        let mut bed2 = Furniture::new(
            "res://assets/furniture/mdl_bed_1x2_001.blend",
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
        let mut alice = Person::new_alloc();
        alice.set_position(Vector3::new(5.0, 0.0, 10.0));
        alice.set_name("alice");

        let mut bob = Person::new_alloc();
        bob.set_position(Vector3::new(7.0, 0.0, 7.0));
        bob.set_name("bob");

        self.add_person(alice);
        self.add_person(bob);
    }

    pub fn add_person(&mut self, mut person: Gd<Person>) {
        person.connect("sig_selected", &self.to_gd().callable("on_person_selected"));
        person.bind_mut().world = Some(self.to_gd());

        self.scn_root.add_child(&person);
        self.people.push(person);
    }

    pub fn add_furniture(&mut self, furniture: Gd<Furniture>) {
        //furniture.connect("sig_selected", &self.to_gd().callable("on_person_selected"));

        self.scn_root.add_child(&furniture);
        self.furniture.push(furniture);
    }

    pub fn select_person(&mut self, person: Option<Gd<Person>>) {
        self.selected_person = person.clone();
        self.ui_taskbar.bind_mut().select_person(person);
    }
}
