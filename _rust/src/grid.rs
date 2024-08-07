use std::{cmp::max, collections::HashSet};

use godot::{
    classes::{GridContainer, IGridContainer},
    prelude::*,
};

use crate::{
    cell::{Cell, Material},
    ruleset::Ruleset,
};

#[derive(GodotClass)]
#[class(base=GridContainer)]
pub struct Grid {
    cells: Array<Gd<Cell>>,
    selected_material: Option<Gd<Material>>,
    ruleset: Gd<Ruleset>,
    game_board: Option<Gd<Object>>,

    base: Base<GridContainer>,
}

#[godot_api]
impl Grid {
    const SPACING: i32 = 40;

    #[func]
    pub fn initialize(&mut self, ruleset: Gd<Ruleset>, game_board: Gd<Object>) {
        self.selected_material = Some(ruleset.bind().default_material());
        self.ruleset = ruleset;
        self.game_board = Some(game_board);
        self.generate();
    }

    #[func]
    fn generate(&mut self) {
        let columns = self.base().get_columns();
        self.cells
            .resize(columns.pow(2) as usize, &Cell::default(self));
        self.fill_default();

        self.base_mut()
            .add_theme_constant_override("v_separation".into(), max(Self::SPACING / columns, 2));
        self.base_mut()
            .add_theme_constant_override("h_separation".into(), max(Self::SPACING / columns, 2));
    }

    pub fn get_ruleset(&self) -> GdRef<Ruleset> {
        self.ruleset.bind()
    }

    #[func]
    pub fn fill_default(&mut self) {
        self.base_mut()
            .get_children()
            .iter_shared()
            .for_each(|mut child| child.queue_free());
        for i in 0..self.cells.len() {
            self.cells.set(i, Cell::default(self))
        }
    }

    pub fn get_neighbors(&self, index: i32) -> Array<Option<Gd<Cell>>> {
        let total_columns = self.base().get_columns();
        let row = index / total_columns;
        let column = index % total_columns;
        [
            (-1, 1),  // Northeast
            (-1, 0),  // North
            (-1, -1), // Northwest
            (0, 1),   // East
            (0, -1),  // West
            (1, 1),   // Southeast
            (1, 0),   // South
            (1, -1),  // Southwest
        ]
        .into_iter()
        .map(|(x_offset, y_offset)| {
            let new_row = row + y_offset;
            let new_column = column + x_offset;
            let out_of_bounds = new_row < 0
                || new_row >= total_columns
                || new_column < 0
                || new_column >= total_columns;
            if out_of_bounds {
                None
            } else {
                Some(self.cells.at((new_row * total_columns + column) as usize))
            }
        })
        .collect()
    }

    #[func]
    pub fn next_generation(&mut self) {
        let cell_count = self.cells.len();
        let mut new_cells: Array<Gd<Cell>> = Array::new();
        new_cells.resize(cell_count, &Cell::default(self));

        for index in 0..cell_count {
            let mut cell = self.cells.at(index);
            for rule in self.ruleset.bind().get_rules().iter_shared() {
                cell = rule
                    .bind()
                    .transform(cell, index, self, self.ruleset.clone())
            }
        }
        self.cells = new_cells;
        self.refresh()
    }

    fn refresh(&mut self) {
        let current_children = self.base().get_children();
        let cells_set: Dictionary = self.cells.iter_shared().map(|c| (c, true)).collect();
        for mut child in current_children.iter_shared() {
            if !cells_set.contains_key(child.clone()) {
                let index = child.get_index();
                child.queue_free();
                self.base_mut().remove_child(child);
                let new_cell = self.cells.at(index as usize);
                self.base_mut().add_child(new_cell.clone());
                self.base_mut().move_child(new_cell, index);
            }
        }
    }
}

#[godot_api]
impl IGridContainer for Grid {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            cells: Array::new(),
            selected_material: None,
            ruleset: Ruleset::blank(),
            game_board: None,
            base,
        }
    }
}
