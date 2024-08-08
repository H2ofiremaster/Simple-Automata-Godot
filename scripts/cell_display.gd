class_name CellDisplay extends TextureButton

const SCENE = preload("res://scenes/cell_display.tscn")

signal material_selected(cell_material: CellMaterial);

var cell_material: CellMaterial;

static func create(init_cell_material: CellMaterial, callback: Callable) -> CellDisplay:
	var cell_display := SCENE.instantiate();
	cell_display.cell_material = init_cell_material;
	cell_display.modulate = init_cell_material.color;
	cell_display.material_selected.connect(callback);
	return cell_display;


func _on_pressed() -> void:
	material_selected.emit(cell_material);
