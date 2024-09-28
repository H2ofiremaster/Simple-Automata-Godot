use godot::prelude::*;

mod cell;
mod grid;
mod io;
mod pattern;
mod ruleset;

struct AutomataExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AutomataExtension {}
