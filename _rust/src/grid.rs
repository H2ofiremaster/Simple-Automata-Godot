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
    #[func]
    pub fn initialize(&mut self, ruleset: Gd<Ruleset>, game_board: Gd<Object>) {
        self.ruleset = ruleset;
        self.game_board = Some(game_board);
    }

    pub fn get_ruleset(&self) -> GdRef<Ruleset> {
        self.ruleset.bind()
    }

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
