class_name GameBoard extends HBoxContainer

const BLANK_RULESET = preload("res://resources/blank_ruleset.tres")

@export var ruleset: Ruleset;
@export var cell_display_scene: PackedScene;

@onready var cell_selector: GridContainer = $OuterMargins/Panel/InnerMargins/CellSelectorContainer/CellSelector
@onready var grid: Grid = $GridContainer/GridAspect/Grid
@onready var cell_label: CellLabel = $ButtonContainer/CellLabelMargins/CellLabel

func _ready() -> void:
	await get_tree().create_timer(0.1).timeout;
	grid.initialize(BLANK_RULESET, self);
	_on_ruleset_creator_ruleset_changed(BLANK_RULESET);

# Signals

func _on_ruleset_creator_ruleset_changed(new_ruleset: Ruleset) -> void:
	ruleset = new_ruleset;
	for cell in cell_selector.get_children():
		cell.queue_free();
	for cell_material in ruleset.material_array():
		var cell_display: CellDisplay = CellDisplay.create(cell_material, _on_material_selected);
		cell_selector.add_child(cell_display);
	
	grid.set_ruleset(ruleset);
	grid.selected_material = ruleset.default_material();
	grid.fill_default();
	#print(grid.cells);


func _on_material_selected(cell_material: CellMaterial) -> void:
	grid.selected_material = cell_material;
	# print("Selected CellType: " + str(cell_type));
