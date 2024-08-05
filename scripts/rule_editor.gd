class_name RuleEditor extends Control

const CONDITION_EDITOR = preload("res://scenes/condition_editor.tscn")

@export var parent: RulesetEditor;
@export var ruleset: Ruleset;

var rule: Rule;

@onready var input_pattern: PatternInput = $Margins/SurroundingContainer/MainContainer/Definition/InputPattern
@onready var output_pattern: PatternInput = $Margins/SurroundingContainer/MainContainer/Definition/OutputPattern
@onready var move_up_button: Button = $Margins/SurroundingContainer/PositionContainer/MoveUpButton
@onready var move_down_button: Button = $Margins/SurroundingContainer/PositionContainer/MoveDownButton
@onready var conditions: VBoxContainer = $Margins/SurroundingContainer/MainContainer/Conditions/ConditionContainer/Conditions
@onready var collapse_button: Button = $Margins/SurroundingContainer/MainContainer/Conditions/ConditionContainer/AddContainer/CollapseButton
@onready var copy_button: Button = $Margins/SurroundingContainer/PositionContainer/CopyButton


## Initializes this RuleEditor. 
func initialize(init_parent: RulesetEditor, init_ruleset: Ruleset, init_rule: Rule) -> void:
	parent = init_parent;
	ruleset = init_ruleset;
	rule = init_rule;
	var input_pattern_value := rule.input; 
	input_pattern.initialize(ruleset, input_pattern_value);
	var output_pattern_value := rule.output; 
	output_pattern.initialize(ruleset, output_pattern_value);
	
	for condition in rule.conditions:
		var editor: ConditionEditor = CONDITION_EDITOR.instantiate();
		conditions.add_child(editor);
		editor.initialize(self, ruleset, condition);


func update_cell_names() -> void:
	input_pattern.update_cell_names();
	output_pattern.update_cell_names();
	for condition in conditions.get_children():
		if condition is ConditionEditor:
			condition.pattern_input.update_cell_names();

func delete_condition(editor: ConditionEditor) -> void:
	var target_index := rule.conditions.find(editor.condition);
	if target_index != -1:
		rule.conditions.remove_at(target_index);
	editor.queue_free();

# Signals

func _on_add_condition_button_pressed() -> void:
	var new_condition_editor: ConditionEditor = CONDITION_EDITOR.instantiate();
	var new_condition := Condition.new();
	rule.conditions.append(new_condition);
	conditions.add_child(new_condition_editor);
	new_condition_editor.initialize(self, ruleset, new_condition);


func _on_delete_button_pressed() -> void:
	parent.delete_rule_editor(self);


func _on_move_up_button_pressed() -> void:
	parent.move_rule_editor(self, true);


func _on_move_down_button_pressed() -> void:
	parent.move_rule_editor(self, false);


func _on_collapse_button_toggled(toggled_on: bool) -> void:
	if toggled_on:
		conditions.visible = false;
		collapse_button.text = "v";
	else:
		conditions.visible = true;
		collapse_button.text = "^";


func _on_copy_button_pressed() -> void:
	var index := parent.selected_ruleset.rules.find(rule);
	var clone := rule.clone();
	parent.selected_ruleset.rules.insert(index, clone);
	parent.add_rule_editor(clone, index);
