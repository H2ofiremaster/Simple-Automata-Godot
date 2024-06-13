class_name PatternInput extends PanelContainer

const ERROR_LABEL_MESSAGE: String = "Invalid state definition: ";

@export var ruleset: Ruleset;
@export var default_text: String;
@export var can_invert: bool = false;

var pattern: Pattern;
var selected_cell_type: CellType;

@onready var name_select: OptionButton = $Margins/Container/StatusBar/NameSelect
@onready var expand_button: Button = $Margins/Container/StatusBar/ExpandButton
@onready var state_label: Label = $Margins/Container/LabelContainer/StateLabel
@onready var state_edit: LineEdit = $Margins/Container/StateEdit
@onready var error_label: Label = $Margins/Container/ErrorLabel
@onready var invert_toggle: CheckBox = $Margins/Container/LabelContainer/InvertToggle
@onready var not_label: Label = $Margins/Container/StatusBar/NotLabel


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	name_select.set_item_text(0, default_text);

## Initializes this RuleEditor.
func initialize(init_ruleset: Ruleset, init_pattern: Pattern) -> void:
	ruleset = init_ruleset;
	pattern = init_pattern;
	update_cell_names();
		
	if init_pattern == null:
		pattern = Pattern.new();
	else:
		var current_cell: CellType = ruleset.get_cell(pattern.cell_name);
		var index: int = ruleset.cells.find(current_cell);
		if index == -1: 
			name_select.selected = 0;
		else:
			name_select.selected = index + 2;
			selected_cell_type = current_cell;
		
		var state: Dictionary = pattern.cell_state;
		var state_array: Array[String] = [];
		var keys: Array[String] = []; 
		keys.assign(state.keys());
		var values: Array[String] = []; 
		values.assign(state.values());
		for i in range(keys.size()):
			var state_entry := keys[i] + " = " + values[i];
			state_array.append(state_entry);
		var state_string := ", ".join(PackedStringArray(state_array))
		state_edit.text = state_string;
		if state_string != "":
			expand_button.button_pressed = true;


func update_cell_names() -> void:
	while name_select.item_count > 2:
		name_select.remove_item(2);
	for cell in ruleset.cells:
		name_select.add_item(cell.name);

# Signals

func _on_expand_button_toggled(toggled_on: bool) -> void:
	if toggled_on:
		expand_button.text = " ^ ";
		state_label.visible = true;
		state_edit.visible = true;
		invert_toggle.visible = can_invert;
		var dict := Pattern.parse_state(state_edit.text);
		pattern.cell_state = dict;
	else:
		expand_button.text = " v ";
		state_label.visible = false;
		state_edit.visible = false;
		invert_toggle.visible = false;
		pattern.cell_state = {};


func _on_state_edit_text_submitted(new_text: String) -> void:
	var error := Pattern.validate_state(new_text);
	if error == "":
		var state_dict := Pattern.parse_state(new_text)
		pattern.cell_state = state_dict;
		error_label.visible = false;
	else:
		error_label.visible = true;
		error_label.text = ERROR_LABEL_MESSAGE + error;


func _on_name_select_item_selected(index: int) -> void:
	print("Pattern '%s' updated to:" % pattern)
	if index == 0:
		pattern.cell_name = "";
	else:
		pattern.cell_name = name_select.get_item_text(index);
	print(pattern)
	


func _on_invert_toggle_toggled(toggled_on: bool) -> void:
	pattern.inverted = toggled_on;
	not_label.visible = toggled_on;
	
