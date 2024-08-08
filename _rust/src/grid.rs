use std::cmp::max;

use godot::{
    classes::{GridContainer, IGridContainer},
    prelude::*,
};

use crate::{
    cell::{Cell, CellMaterial},
    ruleset::Ruleset,
};

#[derive(GodotClass)]
#[class(base=GridContainer)]
pub struct Grid {
    pub cell_scene: Gd<PackedScene>,

    cells: Array<Gd<Cell>>,
    #[var]
    pub selected_material: Option<Gd<CellMaterial>>,
    #[var]
    pub ruleset: Gd<Ruleset>,
    cell_label: Option<Variant>,

    base: Base<GridContainer>,
}

#[godot_api]
impl Grid {
    const SPACING: i32 = 40;

    #[func(gd_self)]
    pub fn initialize(mut this: Gd<Self>, ruleset: Gd<Ruleset>, game_board: Variant) {
        godot_print!("Initialize called!");
        {
            godot_print!("Binding #1 Starting...");
            let mut grid = this.bind_mut();
            godot_print!("Binding #1 Successful...");
            grid.selected_material = Some(ruleset.bind().default_material());
            grid.ruleset = ruleset;
            grid.cell_label = Some(game_board);
        }
        godot_print!("Binding #1 dropped.");
        Self::generate(this);
    }

    #[func(gd_self)]
    fn generate(mut this: Gd<Self>) {
        let default_cell = Cell::default(this.clone());
        {
            godot_print!("Binding #3 Starting...");
            let mut grid = this.bind_mut();
            godot_print!("Binding #3 successful...");
            let columns = grid.base().get_columns();
            grid.cells.resize(columns.pow(2) as usize, &default_cell);

            grid.base_mut().add_theme_constant_override(
                "v_separation".into(),
                max(Self::SPACING / columns, 2),
            );
            grid.base_mut().add_theme_constant_override(
                "h_separation".into(),
                max(Self::SPACING / columns, 2),
            );
        }
        godot_print!("Binding #3 dropped...");
        Self::fill_default(this);
    }

    #[func(gd_self)]
    pub fn fill_default(mut this: Gd<Self>) {
        {
            this.bind_mut()
                .base_mut()
                .get_children()
                .iter_shared()
                .for_each(|mut child| child.queue_free());
        }
        let cell_count = this.bind().cells.len();
        for i in 0..cell_count {
            let default_cell = Cell::default(this.clone());
            this.bind_mut().cells.set(i, default_cell.clone());
            this.bind_mut().base_mut().add_child(default_cell);
        }
        godot_print!(
            "Filling grid with default cell {}.",
            Cell::default(this.clone())
        )
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
        new_cells.resize(cell_count, &Cell::default(self.to_gd()));

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

    pub fn update_cell_label(&self, cell: Option<Gd<Cell>>, material_changed: bool) {
        let Some(cell_label) = self.cell_label.clone() else {
            godot_error!("[Grid::update_cell_label]: 'cell_label' is initialized.");
            return;
        };
        let cell_variant = cell.map(|c| c.to_variant()).unwrap_or(Variant::nil());
        if material_changed {
            cell_label.call("update", &[cell_variant]);
        } else {
            cell_label.call("update_states", &[cell_variant]);
        }
    }
}

#[godot_api]
impl IGridContainer for Grid {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            cell_scene: PackedScene::new_gd(),
            cells: Array::new(),
            selected_material: None,
            ruleset: Ruleset::blank(),
            cell_label: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.cell_scene = load(Cell::SCENE_PATH);
    }
}
