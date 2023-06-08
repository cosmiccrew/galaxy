use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use galaxy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<EngineState>()
        .add_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cosmic Crew: Galaxy".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),

                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(WorldInspectorPlugin::default().run_if(
            bevy::input::common_conditions::input_toggle_active(true, KeyCode::Slash),
        ))
        .add_plugin(StateInspectorPlugin::<EngineState>::default().run_if(
            bevy::input::common_conditions::input_toggle_active(true, KeyCode::Slash),
        ))
        .add_plugin(GalaxyPlayerPlugin)
        .add_plugin(GalaxyLoadingPlugin)
        .add_plugin(GalaxyPolarPlugin)
        .add_plugin(GalaxyWorldPlugin)
        .run();
}
