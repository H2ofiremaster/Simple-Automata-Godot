class_name Pattern extends Resource

@export var cell_name: String;
@export var cell_state: Dictionary;
@export var inverted: bool;

func _to_string() -> String:
	var prefix := "!" if inverted else "";
	return "Pattern(%s%s%s)" % [prefix, cell_name, cell_state]

func matches(cell: Cell) -> bool:
	var is_match := _matches_absolute(cell);
	return not is_match if inverted else is_match;

func _matches_absolute(cell: Cell) -> bool:
	if not cell:
		return false;
	if cell_name and cell.type.name != cell_name:
		return false;
	if cell_state:
		for key: String in cell_state.keys():
			if not cell.state.has(key):
				return false
			if cell.state[key] != cell_state[key]:
				return false;
	return true;

func clone() -> Pattern:
	var pattern := Pattern.new();
	pattern.cell_name = cell_name;
	pattern.cell_state = cell_state.duplicate();
	pattern.inverted = inverted;
	return pattern;

## Validates a given state string.
## Returns an empty string if it's valid, and returns the first offending section if not.
static func validate_state(state_string: String) -> String:
	var trimmed_state := state_string.replace(" ", "");
	if state_string == "": return "";
	if not state_string.contains("="): return state_string;
	
	var states := trimmed_state.split(",");
	for state in states:
		var parts: = state.split("=", false);
		if parts.size() != 2:
			return state;
	return "";

## Parses a given state string, discarding anything invalid with an error message.
static func parse_state(state_string: String) -> Dictionary:
	var trimmed_state := state_string.replace(" ", "");
	if state_string == "": return {};
	if not state_string.contains("="): 
		printerr("State '" + state_string + "' is invalid.");
		return {}; 
	
	var states := trimmed_state.split(",");
	var state_dictionary: Dictionary = {};
	for state in states:
		var parts := state.split("=", false);
		if parts.size() != 2:
			printerr("State '" + state + "' is invalid.")
		else:
			state_dictionary[parts[0]] = parts[1];
	return state_dictionary;
	
