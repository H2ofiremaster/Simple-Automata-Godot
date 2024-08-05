class_name Grid extends GridContainer

const LINE_COLOR = Color.DIM_GRAY;
const SPACING = 40;
const TEST_CELL_TYPE = preload("res://resources/test_cell_type.tres")
const CELL_OUTLINE = preload("res://scenes/cell_outline.tscn")

var cells: Array[Cell];
var selected_cell_type: CellType;
var ruleset: Ruleset;
var game_board: GameBoard;

var last_hovered_index: int = -1;

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
	for child in get_children():
		child.queue_free();
	for i in cells.size():
		cells[i] = Cell.default(self);
		add_child(cells[i]);

func next_generation() -> void:
	var cell_count := cells.size();
	var new_cells: Array[Cell] = [];
	new_cells.resize(cell_count);
	
	for index in range(cell_count):
		#print("---")
		#print("Transforming: %s" % cell)
		var cell := cells[index];
		for rule in ruleset.rules:
			cell = rule.transform(cell, index, self, ruleset)
			if cell != cells[index]: break;
		new_cells[index] = cell;
	cells = new_cells
	refresh();

func get_neighbors(index: int) -> Array[Cell]:
	var row := index / columns;
	var column := index % columns;
	return [
		get_neighbor(row, column, -1, 1), # Northeast
		get_neighbor(row, column, -1, 0), # North
		get_neighbor(row, column, -1, -1), # Northwest
		get_neighbor(row, column, 0, 1), # East
		get_neighbor(row, column, 0, -1), # West
		get_neighbor(row, column, 1, 1), # Southeast
		get_neighbor(row, column, 1, 0), # South
		get_neighbor(row, column, 1, -1) # Southwest
	];

func get_neighbor(row: int, column: int, y_offset: int, x_offset: int) -> Cell:
	var new_row := row + y_offset;
	var new_column := column + x_offset;
	if new_row < 0 or new_row >= columns: return null;
	if new_column < 0 or new_column >= columns: return null;
	
	return cells[(new_row * columns) + new_column];


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
	var current_children := get_children();
	var cells_dict := {};
	for cell in cells: cells_dict[cell] = true;
	for child in current_children:
		if child is Cell and child not in cells_dict:
			var index := child.get_index();
			child.queue_free();
			self.remove_child(child);
			var new_cell := cells[index];
			self.add_child(new_cell);
			self.move_child(new_cell, index);
	if last_hovered_index > -1:
		cells[last_hovered_index].check_hover_status();


func _on_hovered_cell_updated(cell: Cell) -> void:
	game_board.cell_label.update(cell);


func _on_cell_updated(cell: Cell, type_updated: bool) -> void:
	if type_updated: game_board.cell_label.update(cell);
	else: game_board.cell_label.update_states(cell);


