use galaxy::prelude::*;

fn main() {
    App::new()
        .add_state::<EngineState>()
        .add_plugins(GalaxyDefaultPlugins)
        .add_plugins((
            #[cfg(feature = "debug")]
            GalaxyDebugPlugin,
            GalaxyLoadingPlugin,
            GalaxyPlayerPlugin,
            GalaxyUIPlugin,
            GalaxyPolarPlugin,
            GalaxyGamePlugin,
            GalaxyShaderPlugin,
        ))
        .run();
}
