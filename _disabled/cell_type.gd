class_name CellType extends Resource;

## The name of this cell type.
@export var name: String;
## The color of this cell type.
@export var color: Color;
## A dictionary of the states and their possible values.
@export var states: Dictionary;

func default_state() -> Dictionary:
	var result := {};
	for state: String in states:
		if states.get(state).size() > 0:
			result[state] = states.get(state)[0];
	return result;

func _to_string() -> String:
	return name

