class_name CellLabel extends HBoxContainer

@onready var name_label: Label = $Name
@onready var closing_bracket: Label = $ClosingBracket

var selected_cell: Cell;
var selected_index := 0;

func update(cell: Cell) -> void:
	pass
	
func increment() -> void:
	pass
