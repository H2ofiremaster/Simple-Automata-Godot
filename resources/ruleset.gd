class_name Ruleset extends Resource

const PATH: String = "res://resources/ruleset.tres"

@export var name: String;
@export var cells: Array[CellType];
@export var rules: Array[Rule];


func get_cell(cell_name: String) -> CellType:
	for cell in cells:
		if cell.name == cell_name: return cell;
	return null;
