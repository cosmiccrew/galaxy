use bevy::{input::common_conditions::input_toggle_active, window::close_on_esc};
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use crate::prelude::*;

/// Adds helpful features for debugging, like a [WorldInspectorPlugin] and
/// [StateInspectorPlugin]. These can (by default) be toggled by pressing the
/// Backslash (`\`)  keycode.
pub struct GalaxyDebugPlugin;

impl Plugin for GalaxyDebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((
                WorldInspectorPlugin::default()
                    .run_if(input_toggle_active(true, KeyCode::Backslash)),
                StateInspectorPlugin::<EngineState>::default()
                    .run_if(input_toggle_active(true, KeyCode::Backslash)),
            ));
            app.add_systems(Update, close_on_esc);
        }
    }
}
