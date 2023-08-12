use galaxy::prelude::*;

fn main() {
    App::new()
        .add_state::<EngineState>()
        .add_plugins(GalaxyDefaultPlugins)
        .add_plugins((
            GalaxyDebugPlugin,
            GalaxyLoadingPlugin,
            GalaxyPlayerPlugin,
            GalaxyPolarPlugin,
            GalaxyGamePlugin,
            GalaxyShaderPlugin,
        ))
        .run();
}
