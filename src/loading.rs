use crate::prelude::*;

/// Will be used to load assets when the game starts, so they are all pre-loaded
/// before the game starts.
pub struct GalaxyLoadingPlugin;

impl Plugin for GalaxyLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EngineState::LoadingAssets), Self::setup);
    }
}

impl GalaxyLoadingPlugin {
    fn setup(
        mut commands: Commands,
        query: Query<Entity>,
        mut state: ResMut<NextState<EngineState>>,
        asset_server: Res<AssetServer>,
    ) {
        info!("Setting up the world...");

        commands.spawn((
            Camera2dBundle::default(),
            Persist,
            Name::from("Main Camera"),
        ));

        let _ = asset_server.load_folder("./fonts");
        // let _ = asset_server.load_folder("./shaders");
        // let _ = asset_server.load_folder("./ui");

        state.set(EngineState::MainMenu);

        info!("World has been set up!");
    }
}
