class_name GdRule extends Resource

@export var input: GdPattern = GdPattern.new();
@export var output: GdPattern = GdPattern.new();
@export var conditions: Array[Condition] = [];
@export var editor_color: Color = Color("2c2c2cFF")

## Transforms a cell according to rules in the specified ruleset.
##
## Parameters:
##   cell: The target cell to transform.
##   index: The target cell's neighbors.
##   ruleset: The ruleset to use for the transformation.
## Returns: The transformed cell, or the input cell if it is the same.
func transform(cell: Cell, index: int, grid: Grid, ruleset: Ruleset) -> Cell:
	if not input.matches(cell):
		return cell;
	for condition in conditions:
		if not condition.matches(grid.get_neighbors(index)):
			return cell;
	
	var new_cell := cell.clone();
	if output.cell_name:
		new_cell.type = ruleset.get_cell(output.cell_name);
	if output.cell_state:
		for key: String in output.cell_state.keys():
			if new_cell.state[key] != null:
				new_cell.state[key] = output.cell_state[key];
	
	return new_cell;

func clone() -> Rule:
	var rule := Rule.new();
	rule.input = input.clone();
	rule.output = output.clone();
	@warning_ignore("unassigned_variable")
	var new_conditions: Array[Condition];
	new_conditions.assign(conditions.map(func(c: Condition) -> Condition: return c.clone()));
	rule.conditions = new_conditions;
	rule.editor_color = Color(editor_color);
	return rule;
