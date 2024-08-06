use godot::{
    classes::{display_server::HandleType, ITextureButton, TextureButton},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct Cell {
    scene: Gd<PackedScene>,
    material: Gd<Material>,

    base: Base<TextureButton>,
}

#[godot_api]
impl ITextureButton for Cell {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            scene: load("res://scenes/cell.tscn"),
            material: Material::blank(),
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
    fn blank() -> Gd<Self> {
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
