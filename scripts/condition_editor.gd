class_name ConditionEditor extends Control

enum ConditionType {
	NUMERIC = 1,
	DIRECTIONAL = 2
}

@export var parent: RuleEditor;
@export var ruleset: Ruleset;
@export var condition: Condition;

@onready var pattern_input: PatternInput = $Margins/OuterContainer/InnerPanel/InnerMargins/InnerContainer/PatternInput
@onready var number_input: NumberConditionInput = $Margins/OuterContainer/InnerPanel/InnerMargins/InnerContainer/NumberInput
@onready var directional_input: DirectionalConditionInput = $Margins/OuterContainer/InnerPanel/InnerMargins/InnerContainer/DirectionalInput
@onready var numeric_button: Button = $Margins/OuterContainer/TypeSelector/NumericButton
@onready var directional_button: Button = $Margins/OuterContainer/TypeSelector/DirectionalButton

func _ready() -> void:
	ruleset = load("res://resources/blank_ruleset.tres");
	condition = load("res://resources/condition.tres");
	parent = RuleEditor.new();

## Initializes this condition editor.
func initialize(init_parent: RuleEditor, init_ruleset: Ruleset, init_condition: Condition) -> void:
	parent = init_parent;
	ruleset = init_ruleset;
	condition = init_condition;
	
	#if condition.pattern == null:
		#condition.pattern = Pattern.new();
	
	if condition.condition_type == ConditionType.NUMERIC:
		_on_numeric_button_pressed();
	if condition.condition_type == ConditionType.DIRECTIONAL:
		_on_directional_button_pressed();
	
	pattern_input.initialize(ruleset, condition.pattern);
	number_input.initialize();
	directional_input.initialize();


func _on_numeric_button_pressed() -> void:
	numeric_button.button_pressed = true;
	directional_button.button_pressed = false;
	condition.condition_type = ConditionType.NUMERIC;
	number_input.visible = true;
	directional_input.visible = false;


func _on_directional_button_pressed() -> void:
	directional_button.button_pressed = true;
	numeric_button.button_pressed = false;
	condition.condition_type = ConditionType.DIRECTIONAL;
	directional_input.visible = true;
	number_input.visible = false;
	


func _on_delete_button_pressed() -> void:
	parent.delete_condition(self)
