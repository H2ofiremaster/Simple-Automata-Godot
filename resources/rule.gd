class_name Rule extends Resource

@export var input: Pattern = Pattern.new();
@export var output: Pattern = Pattern.new();
@export var conditions: Array[Condition] = [];

func transform(cell: Cell, neighbors: Array[Cell], ruleset: Ruleset) -> bool:
	# print("Transforming: %s" % cell)
	if not input.matches(cell, ruleset):
		# print("Pattern %s does not match." % input)
		return false;
	# print("Pattern %s matches." % input)
	for condition in conditions:
		if not condition.matches(cell, neighbors, ruleset):
			# print("Condition does not match: %s" % condition)
			return false;
	if output.cell_name:
		# print("Updating cell name: %s -> %s" % [cell.name, output.cell_name]);
		cell.type = ruleset.get_cell(output.cell_name);
	if output.cell_state:
		# print("Updating cell state: %s -> %s" % [cell.state, output.cell_state])
		for key: String in output.cell_state.keys():
			if cell.state[key] != null:
				cell.state[key] = output.cell_state[key];
	if not output.cell_name and not output.cell_state:
		print("%s specifies neither name nor state." % output)
	return true;
