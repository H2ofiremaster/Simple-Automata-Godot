class_name ConditionEditor extends Control

signal delete_requested(to_delete: ConditionEditor);

@export var ruleset: Ruleset;
@export var condition: Condition;

@onready var pattern_input: PatternInput = $Margins/OuterContainer/InnerPanel/InnerMargins/InnerContainer/PatternInput
@onready var number_input: NumberConditionInput = $Margins/OuterContainer/InnerPanel/InnerMargins/InnerContainer/NumberInput
@onready var directional_input: DirectionalConditionInput = $Margins/OuterContainer/InnerPanel/InnerMargins/InnerContainer/DirectionalInput
@onready var numeric_button: Button = $Margins/OuterContainer/TypeSelector/NumericButton
@onready var directional_button: Button = $Margins/OuterContainer/TypeSelector/DirectionalButton

## Initializes this condition editor.
func initialize(init_ruleset: Ruleset, init_condition: Condition) -> void:
	ruleset = init_ruleset;
	condition = init_condition;
	
	if condition.pattern == null:
		condition.pattern = Pattern.new();
	
	pattern_input.initialize(ruleset, condition.pattern);
	number_input.initialize();
	directional_input.initialize();


func _on_numeric_button_pressed() -> void:
	if condition.type == Condition.ConditionType.NUMERIC: return;
	numeric_button.button_pressed = true;
	directional_button.button_pressed = false;
	condition.type = Condition.ConditionType.NUMERIC;
	number_input.visible = true;
	directional_input.visible = false;


func _on_directional_button_pressed() -> void:
	if condition.type == Condition.ConditionType.DIRECTIONAL: return;
	directional_button.button_pressed = true;
	numeric_button.button_pressed = false;
	condition.type = Condition.ConditionType.DIRECTIONAL;
	directional_input.visible = true;
	number_input.visible = false;
	


func _on_delete_button_pressed() -> void:
	delete_requested.emit(self);
