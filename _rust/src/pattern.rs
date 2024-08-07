use godot::prelude::*;

use crate::cell::Cell;

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct Pattern {
    #[export]
    pub cell_material: GString,
    #[export]
    pub cell_state: Dictionary,
    #[export]
    inverted: bool,

    base: Base<Resource>,
}

#[godot_api]
impl Pattern {
    fn create(cell_material: GString, cell_state: Dictionary, inverted: bool) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            cell_material,
            cell_state,
            inverted,
            base,
        })
    }

    pub fn full_clone(&self) -> Gd<Self> {
        Self::create(
            self.cell_material.clone(),
            self.cell_state.duplicate_shallow(),
            self.inverted,
        )
    }

    pub fn matches(&self, cell: Gd<Cell>) -> bool {
        let is_match = self.matches_absolute(cell);
        self.inverted ^ is_match
    }
    fn matches_absolute(&self, cell: Gd<Cell>) -> bool {
        if self.has_material() && cell.bind().material.get_name() != self.cell_material {
            return false;
        }
        if self.has_states() {
            for (key, value) in self.cell_state.iter_shared() {
                if !cell.bind().state.get(key).is_some_and(|v| v == value) {
                    return false;
                }
            }
        }
        true
    }

    pub fn has_material(&self) -> bool {
        !self.cell_material.is_empty()
    }

    pub fn has_states(&self) -> bool {
        !self.cell_state.is_empty()
    }
}

#[godot_api]
impl IResource for Pattern {
    fn to_string(&self) -> GString {
        let prefix = if self.inverted { "!" } else { "" };
        format!(
            "Pattern({prefix}{:?}{:?})",
            self.cell_material, self.cell_state
        )
        .into_godot()
    }

    fn init(base: Base<Self::Base>) -> Self {
        Self {
            cell_material: GString::new(),
            cell_state: Dictionary::new(),
            inverted: false,
            base,
        }
    }
}
