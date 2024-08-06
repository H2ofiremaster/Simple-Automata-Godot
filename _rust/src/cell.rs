use godot::{
    classes::{ITextureButton, TextureButton},
    prelude::*,
};

use crate::grid::Grid;

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct Cell {
    scene: Gd<PackedScene>,
    material: Gd<Material>,
    state: Dictionary,

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
            base,
        })
    }

    pub fn default(grid: &Grid) -> Gd<Self> {
        let default_material = grid.get_ruleset().default_material();
        let default_state = default_material.bind().default_state();
        Self::create(default_material, default_state)
    }
}

#[godot_api]
impl ITextureButton for Cell {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            scene: load(Self::SCENE_PATH),
            material: Material::blank(),
            state: Dictionary::new(),
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

    fn create(name: GString, color: Color, states: Dictionary) -> Option<Gd<Self>> {
        if !states.is_empty() {
            for key in states.keys_array().iter_shared() {
                if !(key.get_type() == VariantType::STRING
                    && states
                        .get(key)
                        .is_some_and(|v| v.get_type() == VariantType::ARRAY))
                {
                    return None;
                }
            }
        }
        Some(Gd::from_init_fn(|base| Material {
            name,
            color,
            states,
            base,
        }))
    }

    pub fn default_state(&self) -> Dictionary {
        let mut result = Dictionary::new();
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
