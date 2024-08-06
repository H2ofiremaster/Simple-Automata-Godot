use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, base=Resource)]
struct Pattern {
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
    #[func]
    fn matches() -> bool {
        todo!()
    }
}

#[godot_api]
impl IResource for Pattern {
    fn to_string(&self) -> GString {
        let prefix = if self.inverted { "!" } else { "" };
        format!("Pattern({prefix}{:?}{:?})", self.cell_name, self.cell_state).into_godot()
    }

    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            cell_name: GString::new(),
            cell_state: Dictionary::new(),
            inverted: false,
            base,
        }
    }
}
