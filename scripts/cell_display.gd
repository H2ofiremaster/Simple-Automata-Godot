class_name CellDisplay extends TextureButton

signal cell_type_selected(cell_type: CellType);

var cell_type: CellType;

func initialize(init_cell_type: CellType) -> void:
	cell_type = init_cell_type;
	self.modulate = cell_type.color;


func _on_pressed() -> void:
	cell_type_selected.emit(cell_type);
