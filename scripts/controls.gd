class_name Controls extends VBoxContainer

@onready var game_board: GameBoard = $"../../../../.."
@onready var grid_size_edit: LineEdit = $GridSizeContainer/GridSizeEdit
@onready var speed_edit: LineEdit = $SpeedControl/SpeedEdit
@onready var speed_slider: HSlider = $SpeedControl/SpeedSlider
@onready var stop_button: Button = $PlaybackButtons/StopButton
@onready var start_button: Button = $PlaybackButtons/StartButton

var step_time: float = 1;
var time_passed: float = 0;
var running: bool = false;

func _process(delta: float) -> void:
	if not running: return;
	time_passed += delta;
	if time_passed >= step_time:
		game_board.grid.next_generation();
		time_passed -= step_time;

func _on_step_button_pressed() -> void:
	running = false;
	game_board.grid.next_generation();



func _on_grid_size_edit_text_submitted(new_text: String) -> void:
	var current_text := grid_size_edit.text;
	if not new_text.is_valid_int():
		grid_size_edit.text = current_text;
	else:
		game_board.grid.columns = int(new_text);
		game_board.grid.generate();
	


func _on_speed_slider_value_changed(value: float) -> void:
	speed_edit.text = str(value);
	step_time = value;


func _on_speed_edit_text_submitted(new_text: String) -> void:
	if new_text.is_valid_float():
		var value := float(new_text);
		speed_edit.text = str(value);
		speed_slider.value = value;
		step_time = value;


func _on_start_button_pressed() -> void:
	running = true;
	time_passed = 0;
	stop_button.visible = true;
	start_button.visible = false;


func _on_stop_button_pressed() -> void:
	running = false;
	stop_button.visible = false;
	start_button.visible = true;
