class_name Pattern extends Resource

@export var cell_name: String;
@export var cell_state: Dictionary;
@export var inverted: bool;

func _to_string() -> String:
	var prefix := "!" if inverted else "";
	return "Pattern(%s%s%s)" % [prefix, cell_name, cell_state]

func matches(cell: Cell, ruleset: Ruleset) -> bool:
	var matches := _matches_absolute(cell, ruleset);
	if inverted:
		return not matches;
	else:
		return matches;

func _matches_absolute(cell: Cell, ruleset: Ruleset) -> bool:
	if not cell:
		return false;
	var name_matches := not cell_name or cell.type.name == cell_name;
	var state_matches := not cell_state;
	for key: String in cell_state.keys():
		if not cell.state.has(key):
			state_matches = false;
			break;
		var current_value: String = cell.state[key];
		var target_value: String = cell_state[key];
		if not current_value:
			state_matches = false;
			break;
		if not current_value == target_value:
			state_matches = false;
			break
	return name_matches and state_matches;

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
	
