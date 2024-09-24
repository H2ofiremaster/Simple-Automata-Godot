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
    cell_label: Option<Gd<Object>>,

    base: Base<GridContainer>,
}

#[godot_api]
impl Grid {
    const SPACING: i32 = 40;

    #[func(gd_self)]
    pub fn initialize(mut this: Gd<Self>, ruleset: Gd<Ruleset>, game_board: Gd<Object>) {
        {
            let mut grid = this.bind_mut();

            let cell_label_object = game_board.get("cell_label".into()).to::<Gd<Object>>();

            grid.selected_material = Some(ruleset.bind().default_material());
            grid.ruleset = ruleset;
            grid.cell_label = Some(cell_label_object);
        }
        Self::generate(this);
    }

    #[func(gd_self)]
    fn generate(mut this: Gd<Self>) {
        let default_cell = Cell::default(this.clone());
        {
            let mut grid = this.bind_mut();
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
        .map(|(y_offset, x_offset)| {
            let new_row = row + y_offset;
            let new_column = column + x_offset;
            let out_of_bounds = new_row < 0
                || new_row >= total_columns
                || new_column < 0
                || new_column >= total_columns;
            if out_of_bounds {
                None
            } else {
                Some(
                    self.cells
                        .at((new_row * total_columns + new_column) as usize),
                )
            }
        })
        .collect()
    }

    #[func(gd_self)]
    pub fn next_generation(mut this: Gd<Self>) {
        let cell_count = this.bind().cells.len();
        let mut new_cell_data: Vec<(Option<Gd<CellMaterial>>, Option<Dictionary>)> = Vec::new();

        for index in 0..cell_count {
            let cell = this.bind().cells.at(index);
            let mut data = (None, None);
            for rule in this.bind().ruleset.bind().get_rules().iter_shared() {
                if let Some(new_data) = rule.bind().transform(
                    cell.clone(),
                    index,
                    this.clone(),
                    this.bind().ruleset.clone(),
                ) {
                    data = new_data
                };
            }
            new_cell_data.push(data);
        }

        for (index, data) in new_cell_data.into_iter().enumerate() {
            let new_material = data.0;
            let new_state = data.1;
            let mut cell = this.bind_mut().cells.at(index);
            let mut cell_binding = cell.bind_mut();
            if let Some(material) = new_material {
                cell_binding.set_material(material);
            }
            if let Some(state) = new_state {
                cell_binding.state = state;
            }
        }
    }

    pub fn update_cell_label(&self, cell: Option<Gd<Cell>>, material_changed: bool) {
        let Some(mut cell_label) = self.cell_label.clone() else {
            godot_error!("[Grid::update_cell_label]: 'cell_label' is initialized.");
            return;
        };
        let cell_variant = cell.map(|c| c.to_variant()).unwrap_or(Variant::nil());
        if material_changed {
            let result = cell_label.try_call("update".into(), &[cell_variant]);
            if let Err(error) = result {
                godot_error!("[Grid::update_cell_label]: 'update' method call failed: {error}")
            }
        } else {
            let result = cell_label.try_call("update_states".into(), &[cell_variant]);
            if let Err(error) = result {
                godot_error!(
                    "[Grid::update_cell_label]: 'update_states' method call failed: {error}"
                )
            }
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
