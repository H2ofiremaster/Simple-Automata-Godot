class_name Grid extends GridContainer

signal cell_type_updated(cell: Cell);
signal cell_state_updated(cell: Cell);

const LINE_COLOR = Color.DIM_GRAY;
const SPACING = 40;
const TEST_CELL_TYPE = preload("res://resources/test_cell_type.tres")
const CELL_OUTLINE = preload("res://scenes/cell_outline.tscn")

var cells: Array[Cell];
var selected_cell_type: CellType;
var ruleset: Ruleset;
var game_board: GameBoard;

func initialize(init_ruleset: Ruleset, init_game_board: GameBoard) -> void:
	self.ruleset = init_ruleset;
	self.game_board = init_game_board;
	self.selected_cell_type = ruleset.default_type();
	generate();


func add_line(point1: Vector2, point2: Vector2) -> void:
	var line: = Line2D.new();
	line.add_point(point1);
	line.add_point(point2);
	line.begin_cap_mode = Line2D.LINE_CAP_BOX;
	line.end_cap_mode = Line2D.LINE_CAP_BOX;
	line.modulate = LINE_COLOR;
	line.z_index = 5;
	self.add_child(line);


func fill_default() -> void:
	for i in cells.size():
		cells[i] = Cell.default(self);
	refresh();

func next_generation() -> void:
	var cell_count := cells.size();
	var new_cells: Array[Cell] = [];
	new_cells.resize(cell_count);
	for i in range(cell_count):
		var cell: Cell = cells[i].clone();
		#print("---")
		#print("Transforming: %s" % cell)
		for rule in ruleset.rules:
			if rule.transform(cell, get_neighbors(i), ruleset):
				break;
		new_cells[i] = cell;
	cells = new_cells
	refresh();

func get_neighbors(index: int) -> Array[Cell]:
	var neighbors: Array[Cell] = [];
	neighbors.append(get_neighbor(index, -1, 1)); # Northeast
	neighbors.append(get_neighbor(index, -1, 0)); # North
	neighbors.append(get_neighbor(index, -1, -1)); # Northwest
	neighbors.append(get_neighbor(index, 0, 1)); # East
	neighbors.append(get_neighbor(index, 0, -1)); # West
	neighbors.append(get_neighbor(index, 1, 1)); # Southeast
	neighbors.append(get_neighbor(index, 1, 0)); # South
	neighbors.append(get_neighbor(index, 1, -1)); # Southwest
	return neighbors;

func get_neighbor(index: int, y_offset: int, x_offset: int) -> Cell:
	#warning-ignore:integer_division
	var row := (index / columns) + y_offset;
	var column := (index % columns) + x_offset;
	if row < 0 or row >= columns: return null;
	if column < 0 or column >= columns: return null;
	
	return cells[index + (columns * y_offset) + x_offset];


func generate() -> void:
	#var _cell_size: Vector2 = self.size / self.columns;
	cells.resize(self.columns * self.columns);
	fill_default();
	
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
		cell.updated.connect(_on_cell_updated);


func _on_hovered_cell_updated(cell: Cell) -> void:
	cell_type_updated.emit(cell);


func _on_cell_updated(cell: Cell, type_updated: bool) -> void:
	if type_updated: cell_type_updated.emit(cell);
	else: cell_state_updated.emit(cell);


