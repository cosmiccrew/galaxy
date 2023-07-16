use crate::prelude::*;

///Will be used to load assets when the game starts, so they are all pre-loaded
/// before the game starts.
pub struct GalaxyLoadingPlugin;

impl Plugin for GalaxyLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(EngineState::LoadingAssets), load_assets)
            .add_systems(
                Update,
                to_next
                    .run_if((in_state(EngineState::LoadingAssets)))
                    .run_if((check_if_loaded)),
            )
            // .add_systems(Update, )
            // .add_system(splash_screen.
            // in_set(OnUpdate(EngineState::LoadingAssets)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(EngineState::LoadingAssets), teardown::<Loaded>);
    }
}

fn setup(mut commands: Commands, query: Query<Entity>) {
    info!("Setting up the world...");

    commands.spawn((Camera2dBundle::default(), Persist));

    //Add the Persist entity to all current items, as these should never be removed
    // by a teardown.
    for entity in &query {
        commands.entity(entity).insert(Persist);
    }

    info!("World has been set up!");
}

fn load_assets(asset_server: Res<AssetServer>) {
    // asset_server.load("planets/planets/planet09.png");
}

fn to_next(mut game_state: ResMut<NextState<EngineState>>) {
    game_state.set(EngineState::InGame);
}

fn check_if_loaded() -> bool {
    true
}