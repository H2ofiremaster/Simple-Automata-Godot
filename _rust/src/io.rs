use std::{fs::write, path::PathBuf};

use godot::{
    builtin::GString,
    classes::{DirAccess, IResource, Resource},
    global::godot_error,
    meta::ToGodot,
    obj::{Base, Gd},
    prelude::{godot_api, GodotClass},
};

use crate::ruleset::{GdRuleset, Ruleset};

#[derive(GodotClass)]
#[class(base=Resource, rename=IO)]
struct GdIO {
    inner: IO,
    base_path: GString,

    base: Base<Resource>,
}

#[godot_api]
impl GdIO {
    const RULESET_PATH: &'static str = "user://rulesets/";

    #[func]
    pub fn save(&self, ruleset: Gd<GdRuleset>) {
        let result = IO::save(
            ruleset.bind().inner(),
            PathBuf::from(self.base_path.to_string()),
        );
        if let Err(error) = result {
            godot_error!("Error saving ruleset '{}': {error}", ruleset.bind().name())
        }
    }
}

#[godot_api]
impl IResource for GdIO {
    fn init(base: Base<Resource>) -> Self {
        let dir = DirAccess::open(GdIO::RULESET_PATH.to_godot());
        let dir = match dir {
            Some(dir) => dir,
            None => {
                DirAccess::make_dir_absolute(GdIO::RULESET_PATH.to_godot());
                DirAccess::open(GdIO::RULESET_PATH.to_godot())
                    .expect("RULESET_PATH has just been created. It should be openable.")
            }
        };
        let base_path = dir.get_current_dir_ex().include_drive(true).done();
        Self {
            inner: IO::default(),
            base_path,
            base,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct IO {
    rulesets: Vec<Ruleset>,
}
impl IO {
    pub fn save(ruleset: &Ruleset, mut base_path: PathBuf) -> Result<(), String> {
        base_path.push(&ruleset.name);
        let ruleset_path = base_path.as_path();
        let rulseset_string = toml::to_string_pretty(ruleset).map_err(|err| format!("{err}"))?;
        write(ruleset_path, rulseset_string).map_err(|err| format!("{err}"))
    }

    pub fn load() {}
}
