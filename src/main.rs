use galaxy::prelude::*;

fn main() {
    let cli = Cli::parse();

    App::new()
        .init_state::<EngineState>()
        .add_plugins(GalaxyDefaultPlugins {
            log_level: cli.log_level,
        })
        .add_plugins((
            #[cfg(feature = "debug")]
            GalaxyDebugPlugin,
            GalaxyLoadingPlugin,
            GalaxyPlayerPlugin,
            GalaxyUIPlugin,
            GalaxyPolarPlugin,
            GalaxyGamePlugin,
            GalaxyPhysicsPlugin,
            // GalaxyShaderPlugin,
        ))
        .run();
}
