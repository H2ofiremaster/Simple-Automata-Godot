class_name GameBoard extends HBoxContainer

@export var ruleset: Ruleset;
@export var cell_display_scene: PackedScene;

@onready var cell_selector: GridContainer = $OuterMargins/Panel/InnerMargins/CellSelectorContainer/CellSelector
@onready var grid: Grid = $GridContainer/GridAspect/Grid
@onready var cell_label: CellLabel = $GridContainer/CellLabel

func _ready() -> void:
	await get_tree().create_timer(0.1).timeout;
	grid.initialize();

func _on_ruleset_creator_ruleset_changed(new_ruleset: Ruleset) -> void:
	ruleset = new_ruleset;
	for cell in cell_selector.get_children():
		cell.queue_free();
	for cell in ruleset.cells:
		var cell_display: CellDisplay = cell_display_scene.instantiate();
		cell_selector.add_child(cell_display);
		cell_display.cell_type_selected.connect(_on_cell_type_selected);
		cell_display.initialize(cell);
		
	grid.fill_default(ruleset);
	print(grid.cells);


func _on_cell_type_selected(cell_type: CellType) -> void:
	grid.selected_cell_type = cell_type;
	print("Selected CellType: " + str(cell_type));



func _on_grid_hovered_cell_updated(cell: Cell) -> void:
	cell_label.update(cell);


