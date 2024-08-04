class_name Rule extends Resource

@export var input: Pattern = Pattern.new();
@export var output: Pattern = Pattern.new();
@export var conditions: Array[Condition] = [];

## Transforms a cell according to rules in the specified ruleset.
##
## Parameters:
##   cell: The target cell to transform.
##   neighbors: The target cell's neighbors.
##   ruleset: The ruleset to use for the transformation.
## Returns: The transformed cell, or the input cell if it is the same.
func transform(cell: Cell, neighbors: Array[Cell], ruleset: Ruleset) -> Cell:
	if not input.matches(cell, ruleset):
		return cell;
	for condition in conditions:
		if not condition.matches(cell, neighbors, ruleset):
			return cell;
	var new_cell := cell.clone();
	if output.cell_name:
		new_cell.type = ruleset.get_cell(output.cell_name);
	if output.cell_state:
		for key: String in output.cell_state.keys():
			if new_cell.state[key] != null:
				new_cell.state[key] = output.cell_state[key];
	if not output.cell_name and not output.cell_state:
		print("%s specifies neither name nor state." % output)
	return new_cell;
