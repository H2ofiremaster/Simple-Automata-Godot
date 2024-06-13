class_name Main extends Control

@onready var ruleset_creator: RulesetCreator = $CanvasLayer/RulesetCreator
@onready var game_board: GameBoard = $GameBoard

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_WINDOWED);


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass
	
func _input(event: InputEvent) -> void:
	if event.is_action_pressed("toggle_fullscreen"):
		var current_mode: = DisplayServer.window_get_mode();
		if current_mode == DisplayServer.WINDOW_MODE_FULLSCREEN:
			DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_WINDOWED);
		else:
			DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_FULLSCREEN);
	if event.is_action_pressed("open_ruleset_creator"):
		ruleset_creator.visible = not ruleset_creator.visible;
		game_board.visible = not game_board.visible;
