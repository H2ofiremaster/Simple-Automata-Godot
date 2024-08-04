class_name RuleEditor extends Control

signal delete_requested(to_delete: RuleEditor);
signal move_requested(mover: RuleEditor, up: bool);

@export var ruleset: Ruleset;
@export var condition_editor_scene: PackedScene;

var rule: Rule;

@onready var input_pattern: PatternInput = $Margins/SurroundingContainer/MainContainer/Definition/InputPattern
@onready var output_pattern: PatternInput = $Margins/SurroundingContainer/MainContainer/Definition/OutputPattern
@onready var move_up_button: Button = $Margins/SurroundingContainer/PositionContainer/MoveUpButton
@onready var move_down_button: Button = $Margins/SurroundingContainer/PositionContainer/MoveDownButton
@onready var conditions: VBoxContainer = $Margins/SurroundingContainer/MainContainer/Conditions/ConditionContainer/Conditions
@onready var collapse_button: Button = $Margins/SurroundingContainer/MainContainer/Conditions/ConditionContainer/AddContainer/CollapseButton


## Initializes this RuleEditor. 
func initialize(init_ruleset: Ruleset, init_rule: Rule) -> void:
	ruleset = init_ruleset;
	rule = init_rule;
	var input_pattern_value := rule.input; 
	input_pattern.initialize(ruleset, input_pattern_value);
	var output_pattern_value := rule.output; 
	output_pattern.initialize(ruleset, output_pattern_value);
	
	for condition in rule.conditions:
		var editor: ConditionEditor = condition_editor_scene.instantiate();
		conditions.add_child(editor);
		editor.delete_requested.connect(_on_condition_delete_requested);
		editor.initialize(ruleset, condition);


func update_cell_names() -> void:
	input_pattern.update_cell_names();
	output_pattern.update_cell_names();
	for condition in conditions.get_children():
		if condition is ConditionEditor:
			condition.pattern_input.update_cell_names();


func _on_add_condition_button_pressed() -> void:
	var new_condition_editor: ConditionEditor = condition_editor_scene.instantiate();
	var new_condition := Condition.new();
	rule.conditions.append(new_condition);
	conditions.add_child(new_condition_editor);
	new_condition_editor.delete_requested.connect(_on_condition_delete_requested);
	new_condition_editor.initialize(ruleset, new_condition);


func _on_condition_delete_requested(to_delete: ConditionEditor) -> void:
	var target_index := rule.conditions.find(to_delete.condition);
	if target_index != -1:
		rule.conditions.remove_at(target_index);
	to_delete.queue_free();


func _on_delete_button_pressed() -> void:
	# print("bb");
	delete_requested.emit(self);


func _on_move_up_button_pressed() -> void:
	move_requested.emit(self, true);


func _on_move_down_button_pressed() -> void:
	move_requested.emit(self, false);


func _on_collapse_button_toggled(toggled_on: bool) -> void:
	if toggled_on:
		conditions.visible = false;
		collapse_button.text = "v";
	else:
		conditions.visible = true;
		collapse_button.text = "^";
