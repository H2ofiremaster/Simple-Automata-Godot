class_name GdRuleset extends Resource

const PATH: String = "res://resources/ruleset.tres"

@export var name: String;
@export var cells: Array[CellType];
@export var rules: Array[Rule];

func default_type() -> CellType:
	return cells[0];

func get_cell(cell_name: String) -> CellType:
	for cell in cells:
		if cell.name == cell_name: return cell;
	return null;
