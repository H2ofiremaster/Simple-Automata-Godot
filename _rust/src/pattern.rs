use godot::prelude::*;

use crate::cell::Cell;

/// A pattern used to match a particular group of cells.
/// Either 'cell_material' or 'cell_state' can be empty to represent anything.
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
        // godot_print!("Testing cell '{cell}' against '{}'.", self.to_string());
        if self.has_material() && cell.bind().get_material().bind().name != self.cell_material {
            // godot_print!(
            //     "Material does not match specified material '{}'.",
            //     self.cell_material,
            // );
            return false;
        }
        if self.has_states() {
            for (key, value) in self.cell_state.iter_shared() {
                if !cell
                    .bind()
                    .state
                    .get(key.clone())
                    .is_some_and(|v| v == value)
                {
                    // godot_print!("Material does not match specified state '{key}: {value}'.",);
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

    /// Validates a given state string.
    /// Returns an empty string if it's valid, and returns the first offending section if not.
    #[func]
    fn validate_state(state_string: String) -> String {
        let trimmed_state = state_string.replace(' ', "");
        if state_string.is_empty() {
            return String::new();
        }
        if !trimmed_state.contains('=') {
            return state_string;
        }

        let states = trimmed_state.split(',');
        for state in states {
            let parts: Vec<&str> = state.split('=').collect();
            if parts.len() != 2 {
                return state.into();
            }
        }

        String::new()
    }

    /// Parses a given state string, discarding anything invalid with an error message.
    /// Use `validate_state` first to handle any errors properly.
    #[func]
    fn parse_state(state_string: String) -> Dictionary {
        let trimmed_state = state_string.replace(' ', "");
        if state_string.is_empty() {
            return Dictionary::new();
        }
        if !trimmed_state.contains('=') {
            godot_error!("State '{state_string}' is invalid.");
            return Dictionary::new();
        }

        let states = trimmed_state.split(',');
        let mut state_dictionary: Dictionary = Dictionary::new();
        for state in states {
            let parts: Vec<&str> = state.split('=').collect();
            if parts.len() != 2 {
                godot_error!("State '{state}' is invalid.")
            } else {
                state_dictionary.set(parts[0], parts[1]);
            }
        }

        state_dictionary
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
