use crate::prelude::*;

///Will be used to load assets when the game starts, so they are all pre-loaded
/// before the game starts.
pub struct GalaxyLoadingPlugin;

impl Plugin for GalaxyLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                OnEnter(EngineState::LoadingAssets),
                (load_assets, splash_setup),
            )
            .add_systems(
                Update,
                (
                    to_next.run_if(if_loaded).run_if(if_timer_finished),
                    tick_timer,
                    rotate_loading_icon,
                )
                    .run_if(in_state(EngineState::LoadingAssets)),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(
                OnTransition {
                    from: EngineState::LoadingAssets,
                    to: EngineState::InGame,
                },
                teardown::<Loaded>,
            );
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

// a `Timer` for the splash screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Component, Reflect)]
struct LoadingIcon;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("planets/parts/light0.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            Loaded,
            LoadingIcon,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Percent(20.),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn rotate_loading_icon(commands: Commands, mut query: Query<&mut Transform, With<LoadingIcon>>) {
    for mut object in &mut query {
        object.rotate_z(10f32.to_radians());
    }
}

// tick the `SplashTimer` timer
fn tick_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.tick(time.delta());
}

// return true if the timer is finished
fn if_timer_finished(mut timer: Res<SplashTimer>) -> bool {
    timer.finished()
}

fn load_assets(asset_server: Res<AssetServer>) {

    // asset_server.load("planets/planets/planet09.png");
}

fn to_next(mut game_state: ResMut<NextState<EngineState>>) {
    game_state.set(EngineState::InGame);
}

fn if_loaded() -> bool {
    true
}
