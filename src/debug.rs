use bevy::{input::common_conditions::input_toggle_active, window::close_on_esc};

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
            app.register_type::<PlanetBundle>()
                .add_plugins(EditorPlugin::default())
                .add_systems(Update, close_on_esc);
        }
    }
}
