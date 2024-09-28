use std::{collections::HashMap, fmt::Display};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    cell::{Cell, CellMaterial, GdCellMaterial},
    grid::Grid,
    pattern::{GdPattern, Pattern},
};

#[derive(Debug, Default)]
pub struct CellData<'a> {
    pub material: Option<&'a CellMaterial>,
    pub states: Option<HashMap<&'a str, &'a str>>,
}
impl<'a> CellData<'a> {
    pub fn into_owned(self) -> (Option<CellMaterial>, Option<HashMap<String, String>>) {
        (
            self.material.cloned(),
            self.states.map(|map| {
                map.iter()
                    .map(|(&k, &v)| (k.to_owned(), v.to_owned()))
                    .collect()
            }),
        )
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource, rename=Ruleset)]
pub struct GdRuleset {
    inner: Ruleset,

    base: Base<Resource>,
}

#[godot_api]
impl GdRuleset {
    pub fn wrap(inner: Ruleset) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { inner, base })
    }

    pub fn inner(&self) -> &Ruleset {
        &self.inner
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn rules(&self) -> &[Rule] {
        &self.inner.rules
    }

    /// Returns a new Array containing this ruleset's rules.
    /// Note that this does a lot of cloning and conversions, so it should be used sparingly.
    #[func]
    pub fn rule_array(&self) -> Array<Gd<GdRule>> {
        self.inner.rules.iter().cloned().map(GdRule::wrap).collect()
    }
    /// Returns a new Array containing this ruleset's materials.
    /// Note that this does a lot of cloning and conversions, so it should be used sparingly.
    #[func]
    pub fn material_array(&self) -> Array<Gd<GdCellMaterial>> {
        self.inner
            .materials
            .iter()
            .cloned()
            .map(GdCellMaterial::wrap)
            .collect()
    }

    ///Returns an Array of the names of this ruleset's materials.
    #[func]
    pub fn material_names(&self) -> Array<GString> {
        self.inner
            .materials
            .iter()
            .map(|m| m.name.to_godot())
            .collect()
    }

    #[func]
    pub fn default_material(&self) -> Gd<GdCellMaterial> {
        GdCellMaterial::wrap(self.inner.default_material())
    }

    #[func]
    pub fn parse_material(&self, name: String) -> Option<Gd<GdCellMaterial>> {
        let material = self.inner.get_material(&name);
        material.cloned().map(GdCellMaterial::wrap)
    }

    #[func]
    pub fn rule_count(&self) -> i32 {
        self.inner.rules.len() as i32
    }

    /// Returns the index of the specified rule in this ruleset, or '-1' if it is not found.
    #[func]
    pub fn rule_index(&self, rule: Gd<GdRule>) -> i32 {
        self.inner
            .rules
            .iter()
            .position(|r| r == rule.bind().inner())
            .map(|a| a as i32)
            .unwrap_or(-1)
    }

    #[func]
    pub fn swap_rules(&mut self, original: i32, new: i32) {
        self.inner.rules.swap(original as usize, new as usize);
    }

    #[func]
    pub fn remove_material(&mut self, material: Gd<GdCellMaterial>) {
        let index = self
            .inner
            .materials
            .iter()
            .position(|m| m == material.bind().inner());
        if let Some(index) = index {
            self.inner.materials.remove(index);
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Ruleset {
    pub name: String,
    materials: Vec<CellMaterial>,
    pub rules: Vec<Rule>,
}

impl Ruleset {
    pub fn create(name: String, materials: Vec<CellMaterial>, rules: Vec<Rule>) -> Self {
        Self {
            name,
            materials,
            rules,
        }
    }
    pub fn blank() -> Self {
        Self::create("Blank".into(), vec![CellMaterial::blank()], Vec::new())
    }

    pub fn default_material(&self) -> CellMaterial {
        self.materials
            .get(0)
            .cloned()
            .unwrap_or(CellMaterial::blank())
    }

    pub fn get_material(&self, name: &str) -> Option<&CellMaterial> {
        self.materials
            .iter()
            .find(|material| &material.name == name)
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource, rename=Rule)]
pub struct GdRule {
    inner: Rule,

    base: Base<Resource>,
}

#[godot_api]
impl GdRule {
    pub fn wrap(inner: Rule) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { inner, base })
    }

    pub fn inner(&self) -> &Rule {
        &self.inner
    }

    pub fn transform<'a>(
        &'a self,
        cell: Gd<Cell>,
        index: usize,
        grid: Gd<Grid>,
        ruleset: &'a Ruleset,
    ) -> Option<CellData> {
        self.inner.transform(cell, index, grid, ruleset)
    }

    #[func]
    pub fn full_clone(&self) -> Gd<Self> {
        Self::wrap(self.inner.clone())
    }

    #[func]
    pub fn editor_color(&self) -> Color {
        self.inner.editor_color
    }

    #[func]
    pub fn input(&self) -> Gd<GdPattern> {
        GdPattern::wrap(self.inner.input.clone())
    }

    #[func]
    pub fn output(&self) -> Gd<GdPattern> {
        GdPattern::wrap(self.inner.output.clone())
    }

    #[func]
    pub fn condition_array(&self) -> Array<Gd<GdCondition>> {
        self.inner
            .conditions
            .iter()
            .map(|c| GdCondition::wrap(c.clone()))
            .collect()
    }

    #[func]
    pub fn set_editor_color(&mut self, color: Color) {
        self.inner.editor_color = color;
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    input: Pattern,
    output: Pattern,
    conditions: Vec<Condition>,
    editor_color: Color,
}

impl Rule {
    pub fn new(
        input: Pattern,
        output: Pattern,
        conditions: Vec<Condition>,
        editor_color: Color,
    ) -> Self {
        Self {
            input,
            output,
            conditions,
            editor_color,
        }
    }

    pub fn transform<'a>(
        &'a self,
        cell: Gd<Cell>,
        index: usize,
        grid: Gd<Grid>,
        ruleset: &'a Ruleset,
    ) -> Option<CellData<'a>> {
        if !self.input.matches(cell.clone()) {
            return None;
        }
        if self.conditions.iter().any(|condition| {
            let matches = condition.matches(grid.bind().get_neighbors(index as i32));
            !matches
        }) {
            return None;
        }

        let mut new_cell_data = CellData {
            material: None,
            states: None,
        };
        if self.output.has_material() {
            let material = ruleset.get_material(&self.output.cell_material);

            new_cell_data.material = material;
        }
        if self.output.has_state() {
            let new_cell_state: HashMap<&str, &str> = self
                .output
                .cell_state
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str()))
                .collect();
            new_cell_data.states = Some(new_cell_state);
        }
        Some(new_cell_data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum ConditionType {
    Numeric(Vec<u8>),
    Directional(Vec<Direction>),
}
impl Default for ConditionType {
    fn default() -> Self {
        Self::Numeric(Vec::new())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
enum Direction {
    Northeast,
    North,
    Northwest,
    East,
    West,
    Southeast,
    South,
    Southwest,
}
impl Direction {
    fn index(self) -> usize {
        match self {
            Direction::Northeast => 0,
            Direction::North => 1,
            Direction::Northwest => 2,
            Direction::East => 3,
            Direction::West => 4,
            Direction::Southeast => 5,
            Direction::South => 6,
            Direction::Southwest => 7,
        }
    }
    fn from_index(index: u8) -> Option<Self> {
        let direction = match index {
            0 => Self::Northeast,
            1 => Self::North,
            2 => Self::Northwest,
            3 => Self::East,
            4 => Self::West,
            5 => Self::Southeast,
            6 => Self::South,
            7 => Self::Southwest,
            _ => return None,
        };
        Some(direction)
    }
}
impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource, rename=Condition)]
pub struct GdCondition {
    inner: Condition,

    base: Base<Resource>,
}

#[godot_api]
impl GdCondition {
    pub fn wrap(inner: Condition) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { inner, base })
    }

    pub fn matches(&self, neighbors: Array<Option<Gd<Cell>>>) -> bool {
        self.inner.matches(neighbors)
    }

    #[func]
    pub fn direction_array(&self) -> Array<i32> {
        match self.inner.condition_type {
            ConditionType::Numeric(_) => {
                godot_error!("'direction_array' called on Numberic Condition.");
                Array::new()
            }
            ConditionType::Directional(ref directions) => {
                directions.iter().map(|&dir| dir.index() as i32).collect()
            }
        }
    }

    #[func]
    pub fn full_clone(&self) -> Gd<Self> {
        Self::wrap(self.inner.clone())
    }

    #[func]
    pub fn toggle_direction(&mut self, direction: i32, toggle_on: bool) {
        let direction_option = Direction::from_index(direction as u8);
        let Some(direction) = direction_option else {
            godot_error!("Index {direction} passed to 'toggle_direction', which does not correspond to a valid Direction.");
            return;
        };
        match &mut self.inner.condition_type {
            ConditionType::Numeric(_) => {
                godot_error!("'toggle_direction' called on numeric condition")
            }
            ConditionType::Directional(ref mut directions) => {
                if toggle_on {
                    directions.push(direction);
                }
            }
        }
    }
}

#[godot_api]
impl IResource for GdCondition {
    fn to_string(&self) -> GString {
        self.inner.to_string().to_godot()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    condition_type: ConditionType,
    pattern: Pattern,
    display: String,
}

impl Condition {
    fn new(condition_type: ConditionType, pattern: Pattern, display: String) -> Self {
        Self {
            condition_type,
            pattern,
            display,
        }
    }

    fn matches(&self, neighbors: Array<Option<Gd<Cell>>>) -> bool {
        match &self.condition_type {
            ConditionType::Numeric(counts) => {
                let count: u8 = neighbors
                    .iter_shared()
                    .flatten()
                    .map(|cell| self.pattern.matches(cell) as u8)
                    .sum();
                counts.contains(&count)
            }
            ConditionType::Directional(directions) => directions.iter().any(|dir| {
                let neighbor = neighbors.at(dir.index());
                neighbor.is_some_and(|n| self.pattern.matches(n))
            }),
        }
    }
}
impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.condition_type {
            ConditionType::Directional(directions) => {
                const DIRECTION_ABBREVIATIONS: [&str; 8] =
                    ["ne", "n", "nw", "e", "w", "se", "s", "sw"];
                let directions = directions
                    .iter()
                    .map(|dir| DIRECTION_ABBREVIATIONS[dir.index()])
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Directional([{directions}] = {}", self.pattern)
            }
            ConditionType::Numeric(counts) => {
                let counts = counts
                    .iter()
                    .map(|count| count.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Numeric([{counts}] = {}", self.pattern)
            }
        }
    }
}
