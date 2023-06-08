use crate::prelude::*;

///Will be used to load assets when the game starts, so they are all pre-loaded
/// before the game starts.
pub struct GalaxyLoadingPlugin;

impl Plugin for GalaxyLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            load_assets
                .in_schedule(OnEnter(EngineState::LoadingAssets))
                .run_if(run_once()),
        );
        // // While in this state, run the `countdown` system
        // .add_system(splash_screen.
        // in_set(OnUpdate(EngineState::LoadingAssets))) // When exiting
        // the state, despawn everything that was spawned for this screen
        // .add_system(
        //     teardown::<SplashScreen>.
        // in_schedule(OnExit(EngineState::LoadingAssets)), );
    }
}

// #[derive(Component)]
// struct SplashScreen;

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<EngineState>>,
) {
    // asset_server.load("planets/planets/planet09.png");

    commands.spawn(Camera2dBundle::default());

    game_state.set(EngineState::InGame);
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
