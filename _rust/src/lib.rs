use godot::prelude::*;

mod pattern;

struct AutomataExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AutomataExtension {}
