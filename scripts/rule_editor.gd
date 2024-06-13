class_name RuleEditor extends Control

signal delete_requested(to_delete: RuleEditor);

@export var ruleset: Ruleset;
@export var condition_editor_scene: PackedScene;

var rule: Rule;

@onready var input_pattern: PatternInput = $Margins/SurroundingContainer/MainContainer/Definition/InputPattern
@onready var output_pattern: PatternInput = $Margins/SurroundingContainer/MainContainer/Definition/OutputPattern
@onready var add_container: VBoxContainer = $Margins/SurroundingContainer/MainContainer/Conditions/ConditionContainer/AddContainer


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
		add_container.add_child(editor);
		editor.delete_requested.connect(_on_condition_delete_requested);
		editor.initialize(ruleset, condition);


func update_cell_names() -> void:
	input_pattern.update_cell_names();
	output_pattern.update_cell_names();
	for condition in add_container.get_children():
		if condition is ConditionEditor:
			condition.pattern_input.update_cell_names();

func _on_add_condition_button_pressed() -> void:
	var new_condition_editor: ConditionEditor = condition_editor_scene.instantiate();
	var new_condition := Condition.new();
	rule.conditions.append(new_condition);
	add_container.add_child(new_condition_editor);
	new_condition_editor.delete_requested.connect(_on_condition_delete_requested);
	new_condition_editor.initialize(ruleset, new_condition);


func _on_condition_delete_requested(to_delete: ConditionEditor) -> void:
	var target_index := rule.conditions.find(to_delete.condition);
	if target_index != -1:
		rule.conditions.remove_at(target_index);
	to_delete.queue_free();


func _on_delete_button_pressed() -> void:
	print("bb");
	delete_requested.emit(self);
