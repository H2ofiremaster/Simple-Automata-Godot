use godot::{
    classes::{ITextureButton, TextureButton},
    prelude::*,
};

use crate::grid::Grid;

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct Cell {
    scene: Gd<PackedScene>,
    pub material: Gd<Material>,
    pub state: Dictionary,
    #[var]
    pub selected_state_index: u32,

    base: Base<TextureButton>,
}

#[godot_api]
impl Cell {
    const SCENE_PATH: &'static str = "res://scenes/cell.tscn";

    pub fn create(material: Gd<Material>, state: Dictionary) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            scene: load(Self::SCENE_PATH),
            material,
            state,
            selected_state_index: 0,
            base,
        })
    }

    pub fn full_clone(&self) -> Gd<Self> {
        Self::create(
            self.material.bind().full_clone(),
            self.state.duplicate_deep(),
        )
    }

    pub fn default(grid: &Grid) -> Gd<Self> {
        let default_material = grid.get_ruleset().default_material();
        let default_state = default_material.bind().default_state();
        Self::create(default_material, default_state)
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

        let possible_values_variant = self.material.bind().states.at(selected_key.clone());
        let Ok(possible_values) = possible_values_variant.try_to::<Array<GString>>() else {
            godot_error!("Material state dictionary did not have String Array as value. Aborting.");
            return;
        };
        let Ok(current_value) = self.state.at(selected_key.clone()).try_to::<GString>() else {
            godot_error!("Cell state did not have String as value. Aborting.");
            return;
        };

        let Some(current_value_index) = possible_values.find(&current_value, None) else {
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
            .set(selected_key, possible_values.at(target_index))
    }
}

#[godot_api]
impl ITextureButton for Cell {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            scene: load(Self::SCENE_PATH),
            material: Material::blank(),
            state: Dictionary::new(),
            selected_state_index: 0,
            base,
        }
    }
}

/// Represents a material that a cell can be. Contains a name, color, and dictionary mapping state names to all possible values.
/// Parameter `states` must be a [Dictionary] mapping [GString] to an [Array] of [GString]s.
#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Material {
    #[export]
    name: GString,
    #[export]
    color: Color,
    #[export]
    states: Dictionary,

    base: Base<Resource>,
}

#[godot_api]
impl Material {
    pub fn blank() -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            name: "Blank".into_godot(),
            color: Color::WHITE,
            states: Dictionary::new(),
            base,
        })
    }

    fn create(name: GString, color: Color, states: Dictionary) -> Gd<Self> {
        // if !states.is_empty() {
        //     for key in states.keys_array().iter_shared() {
        //         if !(key.get_type() == VariantType::STRING
        //             && states
        //                 .get(key)
        //                 .is_some_and(|v| v.get_type() == VariantType::ARRAY))
        //         {
        //             return None;
        //         }
        //     }
        // }
        Gd::from_init_fn(|base| Material {
            name,
            color,
            states,
            base,
        })
    }

    pub fn full_clone(&self) -> Gd<Self> {
        Self::create(
            self.name.clone(),
            self.color.clone(),
            self.states.duplicate_deep(),
        )
    }

    pub fn default_state(&self) -> Dictionary {
        self.states
            .iter_shared()
            .filter_map(|(key, value)| {
                let value: Array<GString> = value.to();
                value.get(0).map(|v| Some((key, v))).unwrap_or(None)
            })
            .collect()
    }
}

#[godot_api]
impl IResource for Material {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            name: GString::new(),
            color: Color::BLACK,
            states: Dictionary::new(),
            base,
        }
    }

    fn to_string(&self) -> GString {
        format!("Material({},{})", self.name, self.color).into_godot()
    }
}
