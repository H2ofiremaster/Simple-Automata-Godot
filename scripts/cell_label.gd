class_name CellLabel extends HBoxContainer

@onready var name_label: Label = $Name
@onready var state_container: HBoxContainer = $StateContainer

var selected_cell: Cell;
var selected_index := 0;

func update(cell: Cell) -> void:
	if !cell:
		self.modulate = Color.TRANSPARENT;
		return;
	else:
		self.modulate = Color.WHITE;
	name_label.text = cell.type.name;
	for child in state_container.get_children():
		child.queue_free();
	for key: String in cell.state.keys():
		var label := Label.new();
		label.text = "%s: %s," % [key, cell.state[key]];
		state_container.add_child(label)
	update_states(cell);

func update_states(cell: Cell) -> void:
	var real_children: Array[Label] = [];
	for child in state_container.get_children():
		if not child.is_queued_for_deletion():
			real_children.append(child);
	for i: int in cell.state.keys().size():
		var key: String = cell.state.keys()[i];
		var label := real_children[i];
		label.text = "%s: %s," % [key, cell.state[key]];
		label.modulate = Color.WHITE;
	if real_children.size() != 0:
		real_children[cell.selected_state_index].modulate = Color.YELLOW;


func _on_grid_cell_state_updated(cell: Cell) -> void:
	update_states(cell);


func _on_grid_cell_type_updated(cell: Cell) -> void:
	update(cell);
