use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use crate::prelude::*;

/// Backslash = KeyCode::Backslash
///
/// Adds helpful features for debugging, like a [WorldInspectorPlugin] and
/// [StateInspectorPlugin]. These can (by default) be toggled by pressing the
/// Backslash (`\`)  keycode.
pub struct GalaxyDebugPlugin;

impl Plugin for GalaxyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Backslash)),
        )
        .add_plugin(
            StateInspectorPlugin::<EngineState>::default()
                .run_if(input_toggle_active(true, KeyCode::Backslash)),
        );
    }
}
