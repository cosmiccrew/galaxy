use galaxy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<EngineState>()
        .add_state::<GameState>()
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                max_wait: bevy::utils::Duration::from_millis(1000),
            },
            ..default()
        })
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
                })
                .set({
                    use bevy::log::LogPlugin;
                    if cfg!(debug_assertions) {
                        LogPlugin {
                            level: bevy::log::Level::DEBUG,
                            filter: "debug,wgpu_core=warn,wgpu_hal=warn,naga=info,bevy=info,\
                                     bevy_diagnostic=debug"
                                .into(),
                        }
                    } else {
                        // this code is compiled only if debug assertions are disabled (release
                        // mode)
                        LogPlugin {
                            level: bevy::log::Level::INFO,
                            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                        }
                    }
                }),
        )
        .add_plugin(GalaxyDebugPlugin)
        .add_plugin(GalaxyLoadingPlugin)
        .add_plugin(GalaxyPlayerPlugin)
        .add_plugin(GalaxyPolarPlugin)
        .add_plugin(GalaxyWorldPlugin)
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .run();
}
