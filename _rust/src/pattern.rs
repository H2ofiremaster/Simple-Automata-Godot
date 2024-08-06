use godot::prelude::*;

use crate::cell::Cell;

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct Pattern {
    #[export]
    cell_name: GString,
    #[export]
    cell_state: Dictionary,
    #[export]
    inverted: bool,

    base: Base<Resource>,
}

#[godot_api]
impl Pattern {
    fn create(cell_name: GString, cell_state: Dictionary, inverted: bool) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            cell_name,
            cell_state,
            inverted,
            base,
        })
    }

    pub fn full_clone(&self) -> Gd<Self> {
        Self::create(
            self.cell_name.clone(),
            self.cell_state.duplicate_shallow(),
            self.inverted,
        )
    }

    pub fn matches(&self, cell: Gd<Cell>) -> bool {
        todo!()
    }
}

#[godot_api]
impl IResource for Pattern {
    fn to_string(&self) -> GString {
        let prefix = if self.inverted { "!" } else { "" };
        format!("Pattern({prefix}{:?}{:?})", self.cell_name, self.cell_state).into_godot()
    }

    fn init(base: Base<Self::Base>) -> Self {
        Self {
            cell_name: GString::new(),
            cell_state: Dictionary::new(),
            inverted: false,
            base,
        }
    }
}
