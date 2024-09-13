use godot::prelude::*;

use crate::{
    cell::{Cell, CellMaterial},
    grid::Grid,
    pattern::Pattern,
};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Ruleset {
    #[export]
    name: GString,
    #[export]
    materials: Array<Gd<CellMaterial>>,
    #[export]
    pub rules: Array<Gd<Rule>>,

    base: Base<Resource>,
}

#[godot_api]
impl Ruleset {
    pub fn create(
        name: GString,
        materials: Array<Gd<CellMaterial>>,
        rules: Array<Gd<Rule>>,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            name,
            materials,
            rules,
            base,
        })
    }
    pub fn blank() -> Gd<Self> {
        Self::create("Blank".into(), array![CellMaterial::blank()], Array::new())
    }

    #[func]
    pub fn default_material(&self) -> Gd<CellMaterial> {
        self.materials.get(0).unwrap_or(CellMaterial::blank())
    }

    #[func]
    pub fn get_material(&self, name: GString) -> Option<Gd<CellMaterial>> {
        self.materials
            .iter_shared()
            .find(|material| material.bind().get_name() == name)
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Rule {
    #[export]
    input: Gd<Pattern>,
    #[export]
    output: Gd<Pattern>,
    #[export]
    conditions: Array<Gd<Condition>>,
    #[export]
    editor_color: Color,

    base: Base<Resource>,
}

#[godot_api]
impl Rule {
    pub fn create(
        input: Gd<Pattern>,
        output: Gd<Pattern>,
        conditions: Array<Gd<Condition>>,
        editor_color: Color,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            input,
            output,
            conditions,
            editor_color,
            base,
        })
    }

    #[func]
    pub fn full_clone(&self) -> Gd<Self> {
        Self::create(
            self.input.bind().full_clone(),
            self.output.bind().full_clone(),
            self.conditions
                .iter_shared()
                .map(|c| c.bind().full_clone())
                .collect(),
            self.editor_color,
        )
    }

    pub fn transform(
        &self,
        cell: Gd<Cell>,
        index: usize,
        grid: Gd<Grid>,
        ruleset: Gd<Ruleset>,
    ) -> Option<(Option<Gd<CellMaterial>>, Option<Dictionary>)> {
        if !self.input.bind().matches(cell.clone()) {
            // godot_print!(
            //     "'{cell}' does not match input material '{}'. Aborting.",
            //     self.input
            // );
            return None;
        }
        if self.conditions.iter_shared().any(|condition| {
            let matches = condition
                .bind()
                .matches(grid.bind().get_neighbors(index as i32));
            // godot_print!("Condition '{}' matches: {matches}", condition);
            !matches
        }) {
            // godot_print!(
            //     "'{cell}' does not match one or more of '{}'. Aborting.",
            //     self.conditions
            // );
            return None;
        }

        let mut new_cell_data = (None, None);
        if self.output.bind().has_material() {
            let material = ruleset
                .bind()
                .get_material(self.output.bind().get_cell_material());

            new_cell_data.0 = material;
            // godot_print!("Transforming material of {cell} to {new_cell}.");
        }
        if self.output.bind().has_states() {
            let mut new_cell_state: Dictionary = Dictionary::new();
            self.output
                .bind()
                .get_cell_state()
                .iter_shared()
                .for_each(|(key, value)| new_cell_state.set(key, value));
            new_cell_data.1 = Some(new_cell_state);
            // godot_print!("Transforming states of {cell} to {new_cell_state}.");
        }
        Some(new_cell_data)
    }
}

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy)]
#[godot(via=u8)]
enum ConditionType {
    Numeric,
    Directional,
}
impl Default for ConditionType {
    fn default() -> Self {
        Self::Numeric
    }
}

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy)]
#[godot(via=u8)]
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
impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Condition {
    #[export]
    condition_type: ConditionType,
    #[export]
    directions: Array<u8>,
    #[export]
    counts: Array<u8>,
    #[export]
    pattern: Gd<Pattern>,
    #[export]
    display: GString,

    base: Base<Resource>,
}

#[godot_api]
impl Condition {
    fn create(
        condition_type: ConditionType,
        directions: Array<u8>,
        counts: Array<u8>,
        pattern: Gd<Pattern>,
        display: GString,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            condition_type,
            directions,
            counts,
            pattern,
            display,
            base,
        })
    }

    fn full_clone(&self) -> Gd<Self> {
        Self::create(
            self.condition_type,
            self.directions.duplicate_shallow(),
            self.counts.duplicate_shallow(),
            self.pattern.bind().full_clone(),
            self.display.clone(),
        )
    }

    fn matches(&self, neighbors: Array<Option<Gd<Cell>>>) -> bool {
        // godot_print!(
        //     "Testing condition: {}, type: {:?}, neighbors: {:?}",
        //     self.to_string(),
        //     self.condition_type,
        //     neighbors
        //         .iter_shared()
        //         .map(|o| o.map(|c| c.to_string()))
        //         .collect::<Vec<_>>(),
        // );
        match self.condition_type {
            ConditionType::Numeric => {
                let count: u8 = neighbors
                    .iter_shared()
                    .flatten()
                    .map(|cell| self.pattern.bind().matches(cell) as u8)
                    .sum();
                self.counts.contains(&count)
            }
            ConditionType::Directional => {
                // godot_print!(
                //     "Testing directional condition with directions: {}",
                //     self.directions
                // );
                //
                self.directions.iter_shared().any(|dir| {
                    let neighbor = neighbors.at(usize::from(dir) - 1);
                    let matches = neighbor
                        .clone()
                        .is_some_and(|n| self.pattern.bind().matches(n));
                    // godot_print!(
                    //     "Testing direction {dir}: Neighbor: {:?}, Matches '{}': {matches}",
                    //     neighbor.map(|c| c.to_string()),
                    //     self.pattern,
                    // );
                    matches
                })
            }
        }
    }
}

#[godot_api]
impl IResource for Condition {
    fn to_string(&self) -> GString {
        match self.condition_type {
            ConditionType::Directional => {
                const DIRECTION_ABBREVIATIONS: [&str; 8] =
                    ["ne", "n", "nw", "e", "w", "se", "s", "sw"];
                let directions = self
                    .directions
                    .iter_shared()
                    .map(|dir| DIRECTION_ABBREVIATIONS[dir as usize])
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Directional([{directions}] = {}", self.pattern).into_godot()
            }
            ConditionType::Numeric => {
                let counts = self
                    .counts
                    .iter_shared()
                    .map(|count| count.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Numeric([{counts}] = {}", self.pattern).into_godot()
            }
        }
    }
}
