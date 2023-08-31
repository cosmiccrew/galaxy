use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    window::close_on_esc,
};

use crate::prelude::*;

// /// Adds helpful features for debugging, like a [WorldInspectorPlugin] and
// /// [StateInspectorPlugin]. These can (by default) be toggled by pressing the
// /// Backslash (`\`)  keycode.
pub struct GalaxyDebugPlugin;

impl Plugin for GalaxyDebugPlugin {
    fn build(&self, app: &mut App) {
        // use bevy_editor_pls::prelude::*;
        // use bevy_inspector_egui::prelude::*;

        app.register_type::<CelestialBundle<Earthlike>>()
            .register_type::<CloudCover>();

        app.add_plugins((
            // EditorPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
        ))
        .add_systems(Update, close_on_esc);

        #[cfg(not(target_family = "wasm"))]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default());
    }
}
