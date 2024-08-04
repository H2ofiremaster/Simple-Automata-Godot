class_name Condition extends Resource

@export var type: ConditionType = ConditionType.NUMERIC;
@export var directions: Array[Direction] = [];
@export var counts: Array[int] = [];
@export var pattern: Pattern = Pattern.new();
@export var display: String = "";

enum ConditionType {
	NUMERIC,
	DIRECTIONAL,
}

enum Direction {
	NORTHEAST = 0,
	NORTH = 1,
	NORTHWEST = 2,
	EAST = 3,
	WEST = 4,
	SOUTHEAST = 5,
	SOUTH = 6,
	SOUTHWEST = 7,
}

func _to_string() -> String:
	match type:
		ConditionType.DIRECTIONAL:
			var direction_abbreviations: Array[String] = [];
			for direction in directions:
				match direction:
					Direction.NORTHEAST:
						direction_abbreviations.append("ne");
					Direction.NORTH:
						direction_abbreviations.append("n");
					Direction.NORTHWEST:
						direction_abbreviations.append("nw");
					Direction.EAST:
						direction_abbreviations.append("e");
					Direction.WEST:
						direction_abbreviations.append("w");
					Direction.SOUTHEAST:
						direction_abbreviations.append("se");
					Direction.SOUTH:
						direction_abbreviations.append("s");
					Direction.SOUTHWEST:
						direction_abbreviations.append("sw");
			return "Directional(%s = %s)" % [direction_abbreviations, pattern]
		ConditionType.NUMERIC:
			return "Numeric(%s = %s)" % [counts, pattern]
	return "None"

func matches(cell: Cell, neighbors: Array[Cell], ruleset: Ruleset) -> bool:
	match type:
		ConditionType.NUMERIC:
			var count: int = 0;
			for neighbor in neighbors:
				if pattern.matches(neighbor, ruleset):
					count += 1;
			#print("Cell '%s' has '%d' matching neighbors. Needs '%s' Matches: %s" % [cell, count, counts, count in counts])
			return count in counts;
		ConditionType.DIRECTIONAL:
			for direction: Direction in directions:
				if pattern.matches(neighbors[direction], ruleset):
					return true;
			return false;
	return false;
