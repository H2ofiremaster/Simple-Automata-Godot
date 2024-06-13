class_name Cell extends TextureButton

signal hovered_cell_updated(cell: Cell);
signal type_change_requested(cell: Cell);
signal type_clear_requested(cell: Cell);
signal state_change_requested(cell: Cell);

const CELL = preload("res://scenes/cell.tscn")

var type: CellType:
	set(value):
		self.modulate = value.color;
		type = value;
var state: Dictionary;

var mouse_mask: int = 0b0000;

static func default(ruleset: Ruleset) -> Cell:
	var defualt_type: CellType = ruleset.cells[0];
	var defualt_state: Dictionary = defualt_type.default_state();
	return Cell.create(defualt_type, defualt_state);

static func create(init_type: CellType, init_state: Dictionary) -> Cell:
	var cell: Cell = CELL.instantiate();
	cell.type = init_type;
	cell.state = init_state;
	return cell;

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	mouse_mask = Input.get_mouse_button_mask();

func _to_string() -> String:
	return "Cell(%s%s)" % [type, state]

func clone() -> Cell:
	var cell := CELL.instantiate();
	cell.type = self.type;
	cell.state = self.state.duplicate();
	return cell;

func _on_mouse_entered() -> void:
	if Input.is_action_pressed("set_type"):
		type_change_requested.emit(self);
	if Input.is_action_pressed("clear_type"):
		type_clear_requested.emit(self);
	hovered_cell_updated.emit(self);


func _on_button_down() -> void:
	if Input.is_action_just_pressed("set_type"):
		type_change_requested.emit(self);
	if Input.is_action_just_pressed("clear_type"):
		type_clear_requested.emit(self);
	if Input.is_action_just_pressed("change_state"):
		state_change_requested.emit(self);
