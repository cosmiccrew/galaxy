use crate::prelude::*;

///Will be used to load assets when the game starts, so they are all pre-loaded
/// before the game starts.
pub struct GalaxyLoadingPlugin;

impl Plugin for GalaxyLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            setup.on_startup(),
            load_assets
                .in_schedule(OnEnter(EngineState::LoadingAssets))
                .run_if(run_once()),
            to_next
                .in_set(OnUpdate(EngineState::LoadingAssets))
                .run_if(check_if_loaded),
        ));

        // // While in this state, run the `countdown` system
        // .add_system(splash_screen.
        // in_set(OnUpdate(EngineState::LoadingAssets))) // When exiting
        // the state, despawn everything that was spawned for this screen
        // .add_system(
        //     teardown::<SplashScreen>.
        // in_schedule(OnExit(EngineState::LoadingAssets)), );
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

// fn splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {

//     let icon = asset_server.load("planets/planets/planet09.png");

//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     align_items: AlignItems::Center,
//                     justify_content: JustifyContent::Center,
//                     size: Size::new(Val::Percent(100.0),
// Val::Percent(100.0)),                     ..default()
//                 },
//                 ..default()
//             },
//             SplashScreen,
//         ))
//         .with_children(|parent| {
//             parent.spawn(ImageBundle {
//                 style: Style {
//                     // This will set the logo to be 200px wide, and auto
// adjust its height                     size: Size::new(Val::Px(200.0),
// Val::Auto),                     ..default()
//                 },
//                 image: UiImage::new(icon),
//                 ..default()
//             });
//         });
// }
