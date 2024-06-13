class_name DirectionalConditionInput extends GridContainer



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
		for direction in editor.condition.directions:
			get_button(direction).button_pressed = true;


func get_button(direction: Condition.Direction) -> Button:
	match direction:
		Condition.Direction.NORTH: return north_button;
		Condition.Direction.SOUTH: return south_button;
		Condition.Direction.EAST: return east_button;
		Condition.Direction.WEST: return west_button;
		Condition.Direction.NORTHEAST: return northeast_button;
		Condition.Direction.SOUTHEAST: return southeast_button;
		Condition.Direction.NORTHWEST: return northwest_button;
		Condition.Direction.SOUTHWEST: return southwest_button;
	printerr("Code reached unreachable point!");
	return null;


func toggle_direction(direction: Condition.Direction, toggle_on: bool) -> void:
	var directions := editor.condition.directions;
	var desired_index := directions.find(direction);
	if toggle_on:
		if desired_index == -1:
			directions.append(direction);
	else:
		if desired_index != -1:
			directions.remove_at(desired_index);

func _on_northwest_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.NORTHWEST, toggled_on);


func _on_north_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.NORTH, toggled_on);


func _on_northeast_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.NORTHEAST, toggled_on);


func _on_west_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.WEST, toggled_on);


func _on_east_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.EAST, toggled_on);


func _on_southwest_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.SOUTHWEST, toggled_on);


func _on_south_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.SOUTH, toggled_on);


func _on_southeast_toggled(toggled_on: bool) -> void:
	toggle_direction(Condition.Direction.SOUTHEAST, toggled_on);
