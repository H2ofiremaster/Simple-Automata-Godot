class_name NumberConditionInput extends LineEdit

enum StatementType { ARRAY, RANGE, UPPER_BOUND, LOWER_BOUND }

const VALID_CHARS: Array[String] = ["0", "1", "2", "3", "4", "5", "6", "7", "8"];

@export var editor: ConditionEditor;


func initialize() -> void:
	self.text = editor.condition.display;


func _strip_text(string: String) -> String:
	var new_text := "";
	for character in string:
		if VALID_CHARS.has(character):
			new_text += character;
	return new_text;


func _handle_array(new_text: String) -> bool:
	var array: Array[int] = [];
	var stripped_text := _strip_text(new_text);
	for character in stripped_text:
		if not array.has(int(character)):
			array.append(int(character));
	var numbers := array;
	
	var string_array: PackedStringArray = [];
	for number in numbers:
		string_array.append(str(number));
	var last_index: int = string_array.size() - 1;
	if last_index > 0: string_array[last_index] = "or " + string_array[last_index];
	var number_format := ", ".join(string_array);
	
	editor.condition.counts = numbers;
	editor.number_input.text = number_format;
	return true;


func _handle_upper_bound(new_text: String, delimiter: String) -> bool:
	var stripped_text := _strip_text(new_text);
	if stripped_text.length() != 1: return false;
	var upper_bound := int(stripped_text);
	@warning_ignore("unassigned_variable")
	var numbers: Array[int];
	numbers.assign(range(0, upper_bound + 1));
	var number_format: String;
	if delimiter == "..": number_format = ".." + str(upper_bound);
	else: number_format = str(upper_bound) + " or less";
	
	_update_values(numbers, number_format);
	return true;


func _handle_lower_bound(new_text: String, delimiter: String) -> bool:
	var stripped_text := _strip_text(new_text);
	if stripped_text.length() != 1: return false;
	var lower_bound := int(stripped_text);
	@warning_ignore("unassigned_variable")
	var numbers: Array[int];
	numbers.assign(range(lower_bound, 9));
	var number_format: String;
	if delimiter == "..": number_format = str(lower_bound) + "..";
	else: number_format = str(lower_bound) + " or more";
	
	_update_values(numbers, number_format);
	return true;


func _handle_range(new_text: String, delimiter: String) -> bool:
	var sides := new_text.split("to");
	if sides.size() == 1: sides = new_text.split("-");
	if sides.size() != 2: return false;
	var prefix := _strip_text(sides[0]);
	var suffix := _strip_text(sides[1]);
	if prefix.length() != 1 or suffix.length() != 1: return false;
	var lower_bound := int(prefix);
	var upper_bound := int(suffix);
	@warning_ignore("unassigned_variable")
	var numbers: Array[int];
	numbers.assign(range(lower_bound, upper_bound + 1) as Array[int]);
	var number_format: String 
	if delimiter == "-": number_format = str(lower_bound) + " - " + str(upper_bound);
	else: number_format = str(lower_bound) + " to " + str(upper_bound);
	
	_update_values(numbers, number_format);
	return true;
	


func _update_values(numbers: Array[int], number_format: String) -> void:
	editor.condition.counts = numbers;
	self.text = number_format;


func _on_text_submitted(new_text: String) -> void:
	var types: Array[StatementType] = [];
	var delimiters: Array[String] = [];
	
	if new_text.contains("to"):
		types.append(StatementType.RANGE);
		delimiters.append("to");
	if new_text.contains("-"):
		types.append(StatementType.RANGE);
		delimiters.append("-");
	
	if new_text.contains("or more"):
		types.append(StatementType.LOWER_BOUND);
		delimiters.append("or more");
	if new_text.ends_with(".."):
		types.append(StatementType.LOWER_BOUND);
		delimiters.append("..");
	
	if new_text.contains("or less"):
		types.append(StatementType.UPPER_BOUND);
		delimiters.append("or less");
	if new_text.contains("or fewer"):
		types.append(StatementType.UPPER_BOUND);
		delimiters.append("or fewer");
	if new_text.begins_with(".."):
		types.append(StatementType.UPPER_BOUND);
		delimiters.append("..");
	else:
		types.append(StatementType.ARRAY);
		delimiters.append("");
	
	for i in range(types.size()):
		var type := types[i];
		var delimiter := delimiters[i];
		var is_changed := false;
		match type: 
			StatementType.RANGE:
				is_changed = _handle_range(new_text, delimiter);
				if is_changed: break
			StatementType.LOWER_BOUND:
				is_changed = _handle_lower_bound(new_text, delimiter);
				if is_changed: break
			StatementType.UPPER_BOUND:
				is_changed = _handle_upper_bound(new_text, delimiter);
				if is_changed: break
			StatementType.ARRAY:
				is_changed = _handle_array(new_text);
				if is_changed: break
	
	self.caret_column = self.text.length();
	editor.condition.display = self.text;


func _on_focus_exited() -> void:
	_on_text_submitted(self.text);
