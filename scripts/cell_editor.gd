class_name CellEditor extends PanelContainer

signal delete_requested(to_delete: CellEditor);
signal cell_name_updated()

@export var ruleset: Ruleset;
@export var type: CellType;

# Nodes
@onready var preview: TextureRect = $MarginContainer/MainContainer/PreviewContainer/Preview
@onready var color_select: ColorPickerButton = %ColorSelect; 
@onready var name_input: LineEdit = %NameInput; 
@onready var state_select: OptionButton = %StateSelect; 
@onready var state_name_input: LineEdit = %StateNameInput; 
@onready var state_edit: TextEdit = %StateEdit; 

func initialize(init_ruleset: Ruleset, init_type: CellType) -> void:
	ruleset = init_ruleset;
	type = init_type;
	
	color_select.color = type.color;
	preview.modulate = type.color;
	name_input.text = type.name;
	
	var state_names: Array[String] = [];
	state_names.assign(type.states.keys());
	for state_name: String in state_names:
		state_select.add_item(state_name);

func get_state(key: String) -> Array:
	if not type.states.has(key):
		type.states[key] = [];
	return type.states[key];


func _on_state_select_item_selected(index: int) -> void:
	if index <= 1: 
		state_edit.text = "";
		state_name_input.text = "";
		return;
	var state_name: String = state_select.get_item_text(index);
	var state: Array = get_state(state_name);
	state_edit.text = "\n".join(state);
	state_name_input.text = state_name;
	


func _on_state_name_input_text_submitted(new_text: String) -> void:
	var selected_id: int = state_select.get_selected_id();
	var old_text: String = state_select.get_item_text(selected_id);
	if selected_id <= 1: return; 
	if new_text.length() < 1:
		state_name_input.text = old_text;
		state_name_input.caret_column = old_text.length();
		return;
	
	var selected_state_values := get_state(old_text);
	type.states[new_text] = selected_state_values;
	type.states.erase(old_text);
	state_select.set_item_text(selected_id, new_text);


func _on_state_edit_text_changed() -> void:
	var text: String = state_edit.text;
	var selected_state_id: int = state_select.get_selected_id();
	var selected_state: String = state_select.get_item_text(selected_state_id);
	var lines := text.split("\n", false);
	if lines.size() == 0 or selected_state == "":
		return;
	type.states[selected_state] = Array(lines);


func _on_color_select_popup_closed() -> void:
	type.color = color_select.color;


func _on_color_select_color_changed(color: Color) -> void:
	type.color = color;
	preview.modulate = color;


func _on_name_input_text_changed(new_text: String) -> void:
	type.name = new_text;
	cell_name_updated.emit();


func _on_add_button_pressed() -> void:
	state_select.add_item(state_name_input.text);
	var last_item: int = state_select.item_count - 1;
	state_select.select(last_item);
	_on_state_select_item_selected(last_item);


func _on_remove_button_pressed() -> void:
	var current_index := state_select.selected;
	if current_index <= 1: return;
	state_select.remove_item(current_index);
	state_select.select(0);
	_on_state_select_item_selected(0);


func _on_delete_button_pressed() -> void:
	delete_requested.emit(self);
