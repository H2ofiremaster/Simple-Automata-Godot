class_name Controls extends VBoxContainer

@onready var game_board: GameBoard = $"../../../../.."
@onready var grid_size_edit: LineEdit = $GridSizeContainer/GridSizeEdit

func _on_step_button_pressed() -> void:
	game_board.grid.next_generation(game_board.ruleset);



func _on_grid_size_edit_text_submitted(new_text: String) -> void:
	var current_text := grid_size_edit.text;
	if not new_text.is_valid_int():
		grid_size_edit.text = current_text;
	else:
		game_board.grid.columns = int(new_text);
		game_board.grid.generate(game_board.ruleset);
	
