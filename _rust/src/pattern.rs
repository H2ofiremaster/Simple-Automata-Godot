use std::{collections::HashMap, fmt::Display};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cell::Cell;

#[derive(GodotClass)]
#[class(base=Resource, rename=Pattern)]
pub struct GdPattern {
    inner: Pattern,

    base: Base<Resource>,
}

#[godot_api]
impl GdPattern {
    pub fn wrap(inner: Pattern) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { inner, base })
    }

    pub fn full_clone(&self) -> Gd<Self> {
        Self::wrap(self.inner.clone())
    }

    pub fn matches(&self, cell: Gd<Cell>) -> bool {
        self.inner.matches(cell)
    }

    pub fn has_material(&self) -> bool {
        self.inner.has_material()
    }
    pub fn get_material(&self) -> &str {
        &self.inner.cell_material
    }

    pub fn has_state(&self) -> bool {
        self.inner.has_state()
    }
    pub fn get_state(&self) -> &HashMap<String, String> {
        &self.inner.cell_state
    }

    /// Validates a given state string.
    /// Returns an empty string if it's valid, and returns the first offending section if not.
    #[func]
    pub fn validate_state(state_string: String) -> String {
        Pattern::validate_state(state_string)
    }

    /// Parses a given state string, discarding anything invalid with an error message, and sets the pattern's state to it.
    /// Use `validate_state` first to handle any errors properly.
    #[func]
    pub fn set_state(&mut self, string: String) {
        self.inner.cell_state = Pattern::parse_state(string);
    }

    #[func]
    pub fn get_state_string(&self) -> String {
        self.inner
            .cell_state
            .iter()
            .map(|(key, value)| format!("{key} = {value}"))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Sets the pattern's cell material
    #[func]
    pub fn set_material(&mut self, string: String) {
        self.inner.cell_material = string
    }

    #[func]
    pub fn is_inverted(&self) -> bool {
        self.inner.inverted
    }

    #[func]
    pub fn set_inverted(&mut self, inverted: bool) {
        self.inner.inverted = inverted;
    }
}

#[godot_api]
impl IResource for GdPattern {
    fn to_string(&self) -> GString {
        self.inner.to_string().to_godot()
    }

    fn init(base: Base<Self::Base>) -> Self {
        Self {
            inner: Pattern::default(),
            base,
        }
    }
}

/// A pattern used to match a particular group of cells.
/// Either 'cell_material' or 'cell_state' can be empty to represent anything.

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Pattern {
    pub cell_material: String,
    pub cell_state: HashMap<String, String>,
    inverted: bool,
}

impl Pattern {
    fn new(cell_material: String, cell_state: HashMap<String, String>, inverted: bool) -> Self {
        Self {
            cell_material,
            cell_state,
            inverted,
        }
    }

    pub fn matches(&self, cell: Gd<Cell>) -> bool {
        let is_match = self.matches_absolute(cell);
        self.inverted ^ is_match
    }
    fn matches_absolute(&self, cell: Gd<Cell>) -> bool {
        if self.has_material() && cell.bind().get_material().bind().name() != self.cell_material {
            return false;
        }
        if self.has_state() {
            for (key, value) in self.cell_state.iter() {
                if !cell
                    .bind()
                    .state
                    .get(key.clone())
                    .is_some_and(|v| &v.to::<GString>().to_string() == value)
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn has_material(&self) -> bool {
        !self.cell_material.is_empty()
    }

    pub fn has_state(&self) -> bool {
        !self.cell_state.is_empty()
    }

    /// Validates a given state string.
    /// Returns an empty string if it's valid, and returns the first offending section if not.
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

    fn parse_state(state_string: String) -> HashMap<String, String> {
        let trimmed_state = state_string.replace(' ', "");
        if state_string.is_empty() {
            return HashMap::new();
        }
        if !trimmed_state.contains('=') {
            godot_error!("State '{state_string}' is invalid.");
            return HashMap::new();
        }

        trimmed_state
            .split(',')
            .filter_map(|state| {
                let parts: Vec<&str> = state.split('=').collect();
                if parts.len() != 2 {
                    godot_error!("State '{state}' is invalid.");
                    None
                } else {
                    Some((parts[0].to_owned(), parts[1].to_owned()))
                }
            })
            .collect()
    }
}
impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.inverted { "!" } else { "" };
        write!(
            f,
            "Pattern({prefix}{:?}{:?})",
            self.cell_material, self.cell_state
        )
    }
}
