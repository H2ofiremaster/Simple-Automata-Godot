class_name Rule extends Resource

@export var input: Pattern
@export var output: Pattern
@export var conditions: Array[Condition] = [];

func transform(cell: Cell, neighbors: Array[Cell], ruleset: Ruleset) -> void:
	if not input.matches(cell, ruleset):
		return;
	var i: int = 0;
	for condition in conditions:
		i += 1;
		if not condition.matches(cell, neighbors, ruleset):
			print("%d: Condition '%s' failed, aborting." % [i, condition])
			return;
	if output.cell_name:
		print("Updating cell name: %s -> %s" % [cell.name, output.cell_name]);
		cell.type = ruleset.get_cell(output.cell_name);
	if output.cell_state:
		print("Updating cell state: %s -> %s" % [cell.state, output.cell_state])
		for key: String in output.cell_state.keys():
			if cell.state[key] != null:
				cell.state[key] = output.cell_state[key];
