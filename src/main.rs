use galaxy::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .add_state::<EngineState>()
        .add_plugins(GalaxyDefaultPlugins)
        .add_plugins((
            GalaxyDebugPlugin,
            GalaxyLoadingPlugin,
            GalaxyPlayerPlugin,
            GalaxyPolarPlugin,
            GalaxyGamePlugin,
            GalaxyShaderPlugin,
        ));

    #[cfg(feature = "render_graph")]
    bevy_mod_debugdump::print_schedule_graph(&mut app, PreUpdate);

    #[cfg(not(feature = "render_graph"))]
    app.run();
}
