use std::collections::HashMap;

use godot::{
    classes::{ITextureButton, InputEvent, InputEventMouseButton, TextureButton},
    global::MouseButton,
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::grid::Grid;

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct Cell {
    pub grid: Option<Gd<Grid>>,
    #[var(get, set = set_material)]
    pub material: Gd<GdCellMaterial>,
    #[var]
    pub state: Dictionary,
    #[var]
    pub selected_state_index: u32,

    base: Base<TextureButton>,
}

#[godot_api]
impl Cell {
    pub const SCENE_PATH: &'static str = "res://scenes/cell.tscn";

    pub fn set_material(&mut self, value: Gd<GdCellMaterial>) {
        self.base_mut().set_modulate(value.bind().color());
        self.material = value;
        self.update_state();
    }

    pub fn create(grid: Gd<Grid>, material: Gd<GdCellMaterial>, state: Dictionary) -> Gd<Self> {
        let mut cell: Gd<Cell> = grid.bind().cell_scene.instantiate_as::<Cell>();
        cell.bind_mut().grid = Some(grid);
        cell.bind_mut().set_material(material);
        cell.bind_mut().state = state;
        cell
    }

    pub fn default(grid: Gd<Grid>) -> Gd<Self> {
        let default_material = grid.bind().ruleset.bind().default_material();
        let default_state = default_material.bind().default_state();
        Self::create(grid, default_material, default_state)
    }

    pub fn update_state(&mut self) {
        let mut new_state = self.material.bind().default_state();
        for key in self.state.keys_shared() {
            if new_state.contains_key(key.clone()) {
                new_state.set(key.clone(), self.state.at(key))
            }
        }
        self.state = new_state;
    }

    #[func]
    pub fn cycle_state(&mut self, amount: i32) {
        if self.state.is_empty() {
            return;
        }
        let Some(selected_key) = self
            .state
            .keys_array()
            .get(self.selected_state_index as usize)
        else {
            return;
        };
        let material = self.material.bind();
        let possible_values_option = material
            .states()
            .get(&selected_key.to::<GString>().to_string());
        let Some(possible_values) = possible_values_option else {
            godot_error!("Material state map returned None. Aborting.");
            return;
        };
        let Ok(current_value) = self.state.at(selected_key.clone()).try_to::<GString>() else {
            godot_error!("Cell state did not have String as value. Aborting.");
            return;
        };

        let Some(current_value_index) = possible_values
            .iter()
            .position(|v| v == &current_value.to_string())
        else {
            godot_error!(
                "Could not find index of 'current_value'. \
                This should not happen, as 'current_value' was obtained from this array."
            );
            return;
        };

        let mut target_index = current_value_index
            .checked_add_signed(amount as isize)
            .unwrap_or(possible_values.len() - 1);

        if target_index >= possible_values.len() {
            target_index = 0;
        }
        self.state
            .set(selected_key, possible_values[target_index].clone())
    }

    #[func(gd_self)]
    fn on_mouse_entered(mut this: Gd<Self>) {
        let input = Input::singleton();
        let Some(grid) = this.bind().grid.clone() else {
            godot_error!("[Cell::on_mouse_entered]: 'grid' is not initialized.");
            return;
        };
        let Some(selected_material) = grid.bind().selected_material.clone() else {
            godot_error!("[Cell::on_mouse_entered]: 'grid' has no selected material.");
            return;
        };

        if input.is_mouse_button_pressed(MouseButton::LEFT) {
            this.bind_mut().set_material(selected_material);
        } else if input.is_mouse_button_pressed(MouseButton::RIGHT) {
            let default_material = grid.bind().ruleset.bind().default_material();
            this.bind_mut().set_material(default_material);
        }
        grid.bind().update_cell_label(Some(this), true);
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        if let Some(grid) = self.grid.clone() {
            grid.bind().update_cell_label(None, true);
        }
    }

    #[func(gd_self)]
    fn on_gui_input(mut this: Gd<Self>, event: Gd<InputEvent>) {
        let Ok(mouse_button_event) = event.try_cast::<InputEventMouseButton>() else {
            return;
        };
        if !mouse_button_event.is_pressed() {
            return;
        }

        let Some(grid) = this.bind().grid.clone() else {
            godot_error!("[Cell::on_gui_input]: 'grid' is not initialized.");
            return;
        };

        let material_changed: bool = match mouse_button_event.get_button_index() {
            MouseButton::LEFT => {
                let Some(selected_material) = grid.bind().selected_material.clone() else {
                    godot_error!(
                        "[Cell::on_gui_input]: 'grid' does not have a 'selected_material'."
                    );
                    return;
                };
                this.bind_mut().set_material(selected_material);
                true
            }
            MouseButton::RIGHT => {
                this.bind_mut()
                    .set_material(grid.bind().ruleset.bind().default_material());
                true
            }
            MouseButton::MIDDLE => {
                let mut cell = this.bind_mut();
                cell.selected_state_index += 1;
                if (cell.selected_state_index as usize) >= cell.state.keys_array().len() {
                    cell.selected_state_index = 0;
                }
                false
            }
            MouseButton::WHEEL_UP => {
                this.bind_mut().cycle_state(1);
                false
            }
            MouseButton::WHEEL_DOWN => {
                this.bind_mut().cycle_state(-1);
                false
            }
            _ => return,
        };
        grid.bind().update_cell_label(Some(this), material_changed);
    }
}

#[godot_api]
impl ITextureButton for Cell {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            grid: None,
            material: GdCellMaterial::blank(),
            state: Dictionary::new(),
            selected_state_index: 0,
            base,
        }
    }

    fn to_string(&self) -> GString {
        format!("{}{}", self.material, self.state).into_godot()
    }
}

/// Represents a material that a cell can be. Contains a name, color, and dictionary mapping state names to all possible values.
/// Parameter `states` must be a [Dictionary] mapping [GString] to an [Array] of [GString]s.
#[derive(GodotClass)]
#[class(base=Resource, rename=CellMaterial)]
pub struct GdCellMaterial {
    inner: CellMaterial,

    base: Base<Resource>,
}

#[godot_api]
impl GdCellMaterial {
    pub fn wrap(inner: CellMaterial) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { inner, base })
    }

    pub fn inner(&self) -> &CellMaterial {
        &self.inner
    }

    pub fn blank() -> Gd<Self> {
        Self::wrap(CellMaterial::blank())
    }

    pub fn default_state(&self) -> Dictionary {
        self.inner.default_state()
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn states(&self) -> &HashMap<String, Vec<String>> {
        &self.inner.states
    }

    pub fn color(&self) -> Color {
        self.inner.color
    }
}

#[godot_api]
impl IResource for GdCellMaterial {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            inner: CellMaterial::default(),
            base,
        }
    }

    fn to_string(&self) -> GString {
        self.inner.name.clone().into_godot()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellMaterial {
    pub name: String,
    pub color: Color,
    states: HashMap<String, Vec<String>>,
}

impl CellMaterial {
    pub fn blank() -> Self {
        Self {
            name: String::from("Blank"),
            color: Color::WHITE,
            states: HashMap::new(),
        }
    }

    fn new(name: String, color: Color, states: HashMap<String, Vec<String>>) -> Self {
        Self {
            name,
            color,
            states,
        }
    }

    pub fn default_state(&self) -> Dictionary {
        self.states
            .iter()
            .filter_map(|(key, value)| {
                value
                    .first()
                    .map(|v| Some((key.to_owned(), v.to_owned())))
                    .unwrap_or(None)
            })
            .collect()
    }
}
