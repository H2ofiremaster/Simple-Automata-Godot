class_name Grid extends GridContainer

signal hovered_cell_updated(cell: Cell);
signal scrolled;

const LINE_COLOR = Color.DIM_GRAY;
const SPACING = 40;
const TEST_CELL_TYPE = preload("res://resources/test_cell_type.tres")
const BLANK_RULESET = preload("res://resources/blank_ruleset.tres")
const CELL_OUTLINE = preload("res://scenes/cell_outline.tscn")

var cells: Array[Cell];
var selected_cell_type: CellType;

func initialize() -> void:
	generate(BLANK_RULESET);


func add_line(point1: Vector2, point2: Vector2) -> void:
	var line: = Line2D.new();
	line.add_point(point1);
	line.add_point(point2);
	line.begin_cap_mode = Line2D.LINE_CAP_BOX;
	line.end_cap_mode = Line2D.LINE_CAP_BOX;
	line.modulate = LINE_COLOR;
	line.z_index = 5;
	self.add_child(line);


func add_cell(ruleset: Ruleset) -> Cell:
	var new_cell: Cell = Cell.default(ruleset);
	self.add_child(new_cell);
	return new_cell;
	
func fill_default(ruleset: Ruleset) -> void:
	for i in cells.size():
		cells[i] = Cell.default(ruleset);
	#print(cells)
	refresh();

func next_generation(ruleset: Ruleset) -> void:
	var new_cells: Array[Cell] = [];
	new_cells.resize(cells.size());
	for i in new_cells.size():
		var cell: Cell = cells[i].clone();
		for rule in ruleset.rules:
			rule.transform(cell, get_neighbors(i), ruleset);
		new_cells[i] = cell;
	cells = new_cells
	#print(cells)
	refresh();

func get_neighbors(index: int) -> Array[Cell]:
	var neighbors: Array[Cell] = [];
	neighbors.append(get_neighbor(index, -1, -1));
	neighbors.append(get_neighbor(index, -1, 0));
	neighbors.append(get_neighbor(index, -1, 1));
	neighbors.append(get_neighbor(index, 0, -1));
	neighbors.append(get_neighbor(index, 0, 1));
	neighbors.append(get_neighbor(index, 1, -1));
	neighbors.append(get_neighbor(index, 1, 0));
	neighbors.append(get_neighbor(index, 1, 1));
	return neighbors;

func get_neighbor(index: int, y_offset: int, x_offset: int) -> Cell:
	#warning-ignore:integer_division
	var row := (index / columns) + y_offset;
	var column := (index % columns) + x_offset;
	if row < 0 or row >= columns: return null;
	if column < 0 or column >= columns: return null;
	
	return cells[index + (columns * y_offset) + x_offset];


func generate(ruleset: Ruleset) -> void:
	var _cell_size: Vector2 = self.size / self.columns;
	
	cells.resize(self.columns * self.columns);
	fill_default(ruleset);
	
	add_theme_constant_override("v_separation", max(SPACING / columns, 2));
	add_theme_constant_override("h_separation", max(SPACING / columns, 2));
	#for row: float in range(0, size.y + cell_size.y, cell_size.y):
		#add_line(Vector2(0, row), Vector2(size.x, row));
	#for col: float in range(0, size.x + cell_size.x, cell_size.x):
		#add_line(Vector2(col, 0), Vector2(col, size.y));


func refresh() -> void:
	for child in get_children():
		if child is Cell:
			child.queue_free()
	for cell in cells:
		self.add_child(cell);
		cell.hovered_cell_updated.connect(_on_hovered_cell_updated);
		cell.type_change_requested.connect(_on_cell_type_change_requested);


func _on_hovered_cell_updated(cell: Cell) -> void:
	hovered_cell_updated.emit(cell);


func _on_cell_type_change_requested(cell: Cell) -> void:
	cell.type = selected_cell_type;


