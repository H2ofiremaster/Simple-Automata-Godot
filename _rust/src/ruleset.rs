use godot::prelude::*;

use crate::{
    cell::{Cell, Material},
    grid::Grid,
    pattern::Pattern,
};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Ruleset {
    materials: Array<Gd<Material>>,
    pub rules: Array<Gd<Rule>>,
    base: Base<Resource>,
}

#[godot_api]
impl Ruleset {
    pub fn create(materials: Array<Gd<Material>>, rules: Array<Gd<Rule>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            materials,
            rules,
            base,
        })
    }
    pub fn blank() -> Gd<Self> {
        Self::create(array![Material::blank()], Array::new())
    }

    pub fn default_material(&self) -> Gd<Material> {
        self.materials.get(0).unwrap_or(Material::blank())
    }

    pub fn get_rules(&self) -> &Array<Gd<Rule>> {
        &self.rules
    }

    pub fn get_material(&self, name: GString) -> Option<Gd<Material>> {
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

    base: Base<Resource>,
}

#[godot_api]
impl Rule {
    pub fn create(
        input: Gd<Pattern>,
        output: Gd<Pattern>,
        conditions: Array<Gd<Condition>>,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            input,
            output,
            conditions,
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
        )
    }

    pub fn transform(
        &self,
        cell: Gd<Cell>,
        index: usize,
        grid: &Grid,
        ruleset: Gd<Ruleset>,
    ) -> Gd<Cell> {
        if !self.input.bind().matches(cell.clone()) {
            return cell;
        }
        if self
            .conditions
            .iter_shared()
            .any(|condition| !condition.bind().matches(grid.get_neighbors(index as i32)))
        {
            return cell;
        }

        let mut new_cell: Gd<Cell> = cell.bind().full_clone(grid.to_gd());
        if self.output.bind().has_material() {
            let material = ruleset
                .bind()
                .get_material(self.output.bind().get_cell_material());
            if let Some(material) = material {
                new_cell.bind_mut().material = material;
            }
        }
        if self.output.bind().has_states() {
            let new_cell_state = &mut new_cell.bind_mut().state;
            self.output
                .bind()
                .get_cell_state()
                .iter_shared()
                .for_each(|(key, value)| new_cell_state.set(key, value))
        }
        new_cell
    }
}

#[derive(GodotConvert, Var, Export, Clone, Copy)]
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

#[derive(GodotConvert, Var, Export)]
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

    base: Base<Resource>,
}

#[godot_api]
impl Condition {
    fn create(
        condition_type: ConditionType,
        directions: Array<u8>,
        counts: Array<u8>,
        pattern: Gd<Pattern>,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            condition_type,
            directions,
            counts,
            pattern,
            base,
        })
    }

    fn full_clone(&self) -> Gd<Self> {
        Self::create(
            self.condition_type,
            self.directions.duplicate_shallow(),
            self.counts.duplicate_shallow(),
            self.pattern.bind().full_clone(),
        )
    }

    fn matches(&self, neighbors: Array<Option<Gd<Cell>>>) -> bool {
        match self.condition_type {
            ConditionType::Numeric => {
                let count: u8 = neighbors
                    .iter_shared()
                    .flatten()
                    .map(|cell| self.pattern.bind().matches(cell) as u8)
                    .sum();
                self.counts.contains(&count)
            }
            ConditionType::Directional => self.directions.iter_shared().any(|dir| {
                neighbors
                    .at(dir.into())
                    .is_some_and(|neighbor| self.pattern.bind().matches(neighbor))
            }),
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
