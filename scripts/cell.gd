class_name GdCell extends TextureButton

signal hovered_cell_updated(cell: Cell);
signal updated(cell: Cell, type_updated: bool);

const CELL = preload("res://scenes/cell.tscn")

var type: CellType:
	set(value):
		self.modulate = value.color;
		type = value;
		update_state();
var state: Dictionary;
var grid: Grid;
var selected_state_index: int = 0;

static func default(init_grid: Grid) -> Cell:
	var defualt_type: CellType = init_grid.ruleset.default_type();
	var defualt_state: Dictionary = defualt_type.default_state();
	return Cell.create(defualt_type, defualt_state, init_grid);

static func create(init_type: CellType, init_state: Dictionary, init_grid: Grid) -> Cell:
	var cell: Cell = CELL.instantiate();
	cell.type = init_type;
	cell.state = init_state;
	cell.grid = init_grid;
	cell.hovered_cell_updated.connect(init_grid._on_hovered_cell_updated);
	cell.updated.connect(init_grid._on_cell_updated);
	return cell;


func _to_string() -> String:
	return "Cell(%s%s)" % [type, state]


func cycle_selected_state(amount: int) -> void:
	if state.is_empty(): return;
	var selected_key: String = state.keys()[selected_state_index];
	if !selected_key: return;
	var current_value: String = state[selected_key];
	var possible_states: Array = self.type.states[selected_key];
	var current_value_index: int = possible_states.find(current_value);
	var target_index := current_value_index + amount;
	if target_index >= possible_states.size(): target_index = 0;
	if target_index < 0: target_index = possible_states.size() - 1;
	state[selected_key] = possible_states[target_index];


func clone() -> Cell:
	return Cell.create(self.type, self.state.duplicate(), self.grid);
	# var cell := CELL.instantiate();
	# cell.type = self.type;
	# cell.state = self.state.duplicate();
	# cell.grid = self.grid;
	# return cell;


func update_state() -> void:
	var new_state := type.default_state();
	for key: String in state.keys():
		if new_state.has(key):
			# print("setting '%s' to %s" % [key, state[key]])
			new_state[key] = state[key];
	state = new_state;

func check_hover_status() -> void:
	if not get_viewport(): return;
	if get_global_rect().has_point(get_global_mouse_position()):
		hovered_cell_updated.emit(self);

# Signals

func _on_mouse_entered() -> void:
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		self.type = grid.selected_cell_type;
		updated.emit(self, true);
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT):
		self.type = grid.ruleset.default_type()
		updated.emit(self, true);
	hovered_cell_updated.emit(self);
	grid.last_hovered_index = grid.cells.find(self);


func _on_mouse_exited() -> void:
	hovered_cell_updated.emit(null);


func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		match event.button_index:
			MOUSE_BUTTON_LEFT:
				self.type = grid.selected_cell_type;
				updated.emit(self, true);
			MOUSE_BUTTON_RIGHT:
				self.type = grid.ruleset.default_type();
				updated.emit(self, true);
			MOUSE_BUTTON_MIDDLE:
				self.selected_state_index += 1;
				if self.selected_state_index >= self.state.keys().size():
					self.selected_state_index = 0;
				updated.emit(self, false);
			MOUSE_BUTTON_WHEEL_UP:
				cycle_selected_state(1);
				updated.emit(self, false);
			MOUSE_BUTTON_WHEEL_DOWN:
				cycle_selected_state(-1);
				updated.emit(self, false);



