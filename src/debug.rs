use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    window::close_on_esc,
};

use self::planet::PlanetNormal;
use crate::prelude::*;

// /// Adds helpful features for debugging, like a [WorldInspectorPlugin] and
// /// [StateInspectorPlugin]. These can (by default) be toggled by pressing the
// /// Backslash (`\`)  keycode.
pub struct GalaxyDebugPlugin;

impl Plugin for GalaxyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // EditorPlugin::default(),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
        ))
        .add_systems(Update, close_on_esc)
        .register_type::<PlanetNormal>()
        .register_type::<Direction2d>();

        #[cfg(not(target_family = "wasm"))]
        app.add_plugins((
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            bevy_inspector_egui::quick::StateInspectorPlugin::<GameState>::default(),
            bevy_inspector_egui::quick::StateInspectorPlugin::<EngineState>::default(),
        ));
    }
}
