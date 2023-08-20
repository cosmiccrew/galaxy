use bevy::{
    asset::diagnostic::AssetCountDiagnosticsPlugin,
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    input::common_conditions::input_toggle_active,
    window::close_on_esc,
};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

use crate::prelude::*;

// /// Adds helpful features for debugging, like a [WorldInspectorPlugin] and
// /// [StateInspectorPlugin]. These can (by default) be toggled by pressing the
// /// Backslash (`\`)  keycode.
pub struct GalaxyDebugPlugin;

impl Plugin for GalaxyDebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        {
            use bevy_editor_pls::prelude::*;

            app.register_type::<CelestialBundle<Earthlike>>()
                .register_type::<CloudCover>();

            app.add_plugins((
                EditorPlugin::default(),
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
            ))
            .add_systems(Update, close_on_esc);
        }
    }
}
