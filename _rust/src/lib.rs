use godot::prelude::*;

mod cell;
mod grid;
mod pattern;
mod ruleset;

struct AutomataExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AutomataExtension {}
