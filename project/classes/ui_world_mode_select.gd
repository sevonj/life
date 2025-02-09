##
##
class_name UiWorldModeSelect
extends Node

var _world: World

var _button_play := Button.new()
var _button_buy := Button.new()
var _button_build := Button.new()

func _ready() -> void:
	if not is_instance_valid(_world):
		_world = _find_world()

	var on_mode_set := func (pressed: bool, mode: String) -> void:
		if is_instance_valid(_world) and pressed:
			pass

	_button_play.text = "Play"
	_button_play.toggle_mode = true
	_button_play.tooltip_text = "Switch to play mode"
	_button_play.pressed.connect(on_mode_set.bind("Play"))

	_button_buy.text = "Buy"
	_button_buy.toggle_mode = true
	_button_buy.tooltip_text = "Switch to buy mode (not implemented)"
	_button_buy.disable()
	_button_buy.pressed.connect(on_mode_set.bind("Buy"))

	_button_build.text = "Build"
	_button_build.toggle_mode = true
	_button_build.tooltip_text = "Switch to build mode"
	_button_build.pressed.connect(on_mode_set.bind("Build"))


func _process(_delta: float) -> void:
	if not is_instance_valid(_world):
		return

	var mode := _world.get_view_mode()
	match mode:
		"Play":
			_button_play.set_pressed_no_signal(true)
			_button_buy.set_pressed_no_signal(false)
			_button_build.set_pressed_no_signal(false)
		"Buy":
			_button_play.set_pressed_no_signal(false)
			_button_buy.set_pressed_no_signal(true)
			_button_build.set_pressed_no_signal(false)
		"Build":
			_button_play.set_pressed_no_signal(false)
			_button_buy.set_pressed_no_signal(false)
			_button_build.set_pressed_no_signal(true)
		_:
			push_error("World has unknown view mode: %s" % mode)


func set_world(world: World) -> void:
	self._world = world


func _find_world() -> World:
	var parent := get_parent()
	while true:
		if parent is World or parent == null:
			break
		parent = get_parent()
	return parent
