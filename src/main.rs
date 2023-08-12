use galaxy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<EngineState>()
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                max_wait: bevy::utils::Duration::from_millis(1000),
            },
            ..default()
        })
        .add_plugins(GalaxyDefaultPlugins)
        .add_plugins((
            GalaxyDebugPlugin,
            GalaxyLoadingPlugin,
            GalaxyPlayerPlugin,
            GalaxyPolarPlugin,
            GalaxyGamePlugin,
            GalaxyShaderPlugin,
        ));
}
