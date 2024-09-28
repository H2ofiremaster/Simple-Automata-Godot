class_name DirectionalConditionInput extends GridContainer

enum Direction {
	NORTHEAST,
	NORTH,
	NORTHWEST,
	EAST,
	WEST,
	SOUTHEAST,
	SOUTH,
	SOUTHWEST,
}


@export var editor: ConditionEditor

@onready var northwest_button: Button = $Northwest
@onready var north_button: Button = $North
@onready var northeast_button: Button = $Northeast
@onready var west_button: Button = $West
@onready var east_button: Button = $East
@onready var southwest_button: Button = $Southwest
@onready var south_button: Button = $South
@onready var southeast_button: Button = $Southeast

# Called when the node enters the scene tree for the first time.
func initialize() -> void:
	if editor.condition != null:
		for direction in editor.condition.direction_array():
			get_button(direction).button_pressed = true;


func get_button(direction: Direction) -> Button:
	match direction:
		Direction.NORTH: return north_button;
		Direction.SOUTH: return south_button;
		Direction.EAST: return east_button;
		Direction.WEST: return west_button;
		Direction.NORTHEAST: return northeast_button;
		Direction.SOUTHEAST: return southeast_button;
		Direction.NORTHWEST: return northwest_button;
		Direction.SOUTHWEST: return southwest_button;
	printerr("Code reached unreachable point!");
	return null;


func toggle_direction(direction: Direction, toggle_on: bool) -> void:
	editor.condition.toggle_direction(direction, toggle_on);

func _on_northwest_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.NORTHWEST, toggled_on);
	northwest_button.modulate = Color.AQUA if toggled_on else Color.WHITE;


func _on_north_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.NORTH, toggled_on);
	north_button.modulate = Color.AQUA if toggled_on else Color.WHITE;


func _on_northeast_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.NORTHEAST, toggled_on);
	northeast_button.modulate = Color.AQUA if toggled_on else Color.WHITE;


func _on_west_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.WEST, toggled_on);
	west_button.modulate = Color.AQUA if toggled_on else Color.WHITE;


func _on_east_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.EAST, toggled_on);
	east_button.modulate = Color.AQUA if toggled_on else Color.WHITE;


func _on_southwest_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.SOUTHWEST, toggled_on);
	southwest_button.modulate = Color.AQUA if toggled_on else Color.WHITE;


func _on_south_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.SOUTH, toggled_on);
	south_button.modulate = Color.AQUA if toggled_on else Color.WHITE;


func _on_southeast_toggled(toggled_on: bool) -> void:
	toggle_direction(Direction.SOUTHEAST, toggled_on);
	southeast_button.modulate = Color.AQUA if toggled_on else Color.WHITE;
