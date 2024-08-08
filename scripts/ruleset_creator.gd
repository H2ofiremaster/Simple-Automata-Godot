class_name RulesetEditor extends Control

signal ruleset_changed(new_ruleset: Ruleset);

const RULESET_PATH: String = "user://rulesets/";

@export var cell_editor_scene: PackedScene;
@export var rule_editor_scene: PackedScene;

var selected_ruleset: Ruleset = null;
var removing_cell: bool = false;
var rulesets: Array[Ruleset] = [];

@onready var cells: VBoxContainer = $Background/VBoxContainer/Data/CellPanel/Margins/CellContainer/Scroll/Cells
@onready var rules: VBoxContainer = $Background/VBoxContainer/Data/RulePanel/Margins/RuleContainer/Scroll/Rules
@onready var ruleset_selector: OptionButton = $Background/VBoxContainer/Toolbar/RulesetSelector
@onready var ruleset_name: LineEdit = $Background/VBoxContainer/Toolbar/RulesetName
@onready var add_rule_button: Button = $Background/VBoxContainer/Data/RulePanel/Margins/RuleContainer/AddRuleButton
@onready var add_cell_button: Button = $Background/VBoxContainer/Data/CellPanel/Margins/CellContainer/Toolbar/AddCellButton
@onready var game_board: GameBoard = $"../../GameBoard"


@onready var ruleset_resource: Ruleset = ResourceLoader.load("res://resources/ruleset.tres", "Ruleset");

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var dir := DirAccess.open(RULESET_PATH);
	if dir == null:
		DirAccess.make_dir_absolute(RULESET_PATH);
		dir = DirAccess.open(RULESET_PATH);
	var ruleset_paths := dir.get_files();
	for path in ruleset_paths:
		print(RULESET_PATH + path);
		var resource := ResourceLoader.load(RULESET_PATH + path, "Ruleset");
		rulesets.append(resource);
	for ruleset in rulesets:
		print(ruleset);
		ruleset_selector.add_item(ruleset.name);


func save() -> void:
	if not selected_ruleset: return;
	var error := ResourceSaver.save(selected_ruleset, RULESET_PATH + selected_ruleset.name + ".tres");
	if error != Error.OK:
		printerr(error);

func disable_rule_move_buttons() -> void:
	var i: int = 0;
	for editor: RuleEditor in rules.get_children():
		editor.move_up_button.disabled = false;
		editor.move_down_button.disabled = false;
		if i == 0:
			editor.move_up_button.disabled = true;
		if i == rules.get_child_count() - 1:
			editor.move_down_button.disabled = true;
		i += 1;

## Adds a rule editor for the specified rule
func add_rule_editor(rule: Rule, index: int = -1) -> void:
	var editor: RuleEditor = rule_editor_scene.instantiate();
	rules.add_child(editor);
	editor.initialize(self, selected_ruleset, rule);
	if index > -1:
		rules.move_child(editor, index);
	disable_rule_move_buttons();

func move_rule_editor(editor: RuleEditor, up: bool) -> void:
	var original_index := selected_ruleset.rules.find(editor.rule);
	var new_index := original_index - 1 if up else original_index + 1;
	
	# print("%s | %s" % [original_index, new_index])
	
	if new_index < 0 or new_index >= selected_ruleset.rules.size():
		return;
	
	var item_b := selected_ruleset.rules[new_index];
	selected_ruleset.rules[new_index] = selected_ruleset.rules[original_index];
	selected_ruleset.rules[original_index] = item_b;
	
	rules.move_child(editor, new_index);
	disable_rule_move_buttons();

func delete_rule_editor(to_delete: RuleEditor) -> void:
	var index := selected_ruleset.rules.find(to_delete.rule);
	if index != -1:
		selected_ruleset.rules.remove_at(index);
	to_delete.queue_free();

# Signals

func _on_new_button_pressed() -> void:
	var ruleset_name_text := ruleset_name.text;
	if ruleset_name_text.is_empty(): 
		printerr("Name is empty; returning")
		return;
	selected_ruleset = ruleset_resource.duplicate();
	selected_ruleset.name = ruleset_name_text;
	rulesets.append(selected_ruleset);
	ruleset_selector.add_item(selected_ruleset.name);
	var last_item := ruleset_selector.item_count - 1;
	ruleset_selector.selected = last_item;
	_on_ruleset_selector_item_selected(last_item);


func _on_save_button_pressed() -> void:
	save();


func _on_ruleset_selector_item_selected(index: int) -> void:
	for cell in cells.get_children():
			cell.queue_free();
	for rule in rules.get_children():
		rule.queue_free();
		
	if index < 2:
		ruleset_name.text = "";
		selected_ruleset = null;
		add_cell_button.disabled = true;
		add_rule_button.disabled = true;
		return;
	
	selected_ruleset = rulesets[index - 2];
	ruleset_name.text = selected_ruleset.name;
	add_cell_button.disabled = false;
	add_rule_button.disabled = false;
	for cell_material in selected_ruleset.materials:
		var editor: CellEditor = cell_editor_scene.instantiate();
		cells.add_child(editor);
		editor.delete_requested.connect(_on_cell_delete_requested);
		editor.cell_name_updated.connect(_on_cell_name_updated);
		editor.initialize(selected_ruleset, cell_material);
	for rule in selected_ruleset.rules:
		add_rule_editor(rule);
	disable_rule_move_buttons()


func _on_add_rule_button_pressed() -> void:
	var rule := Rule.new();
	selected_ruleset.rules.append(rule);
	add_rule_editor(rule)


func _on_add_cell_button_pressed() -> void:
	var editor: CellEditor = cell_editor_scene.instantiate();
	var cell_material := CellMaterial.new();
	selected_ruleset.materials.append(cell_material);
	cells.add_child(editor);
	editor.delete_requested.connect(_on_cell_delete_requested);
	editor.initialize(selected_ruleset, cell_material);


func _on_cell_delete_requested(to_delete: CellEditor) -> void:
	# print("a");
	var index := selected_ruleset.materials.find(to_delete.type);
	if index != -1:
		selected_ruleset.materials.remove_at(index);
	to_delete.queue_free();


func _on_delete_button_pressed() -> void:
	var index := rulesets.find(selected_ruleset);
	if index == -1: 
		printerr("Could not find ruleset in array.")
		return;
	rulesets.remove_at(index);
	var directory := DirAccess.open(RULESET_PATH);
	if not directory: return;
	directory.remove(selected_ruleset.name + ".tres");
	var current_index := ruleset_selector.selected;
	ruleset_selector.remove_item(current_index);
	ruleset_selector.selected = 0;
	_on_ruleset_selector_item_selected(0);


func _on_back_button_pressed() -> void:
	save();
	self.hide();
	game_board.show();
	ruleset_changed.emit(selected_ruleset);


func _on_cell_name_updated() -> void:
	for editor: RuleEditor in rules.get_children():
		editor.update_cell_names();
