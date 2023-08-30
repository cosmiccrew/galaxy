use crate::prelude::*;

use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
pub struct MyAssets {
    // This file will be converted to a texture atlas
    // The configuration for that is part of the `.assets` file
    // Type in `assets/full_dynamic_collection.assets.ron`: `TextureAtlas`
    #[asset(key = "ui/icons/prompts")]
    pub ui_icon_prompts: Handle<TextureAtlas>,

    #[asset(key = "dummy")]
    pub dummy: Handle<Image>,

    #[asset(key = "loading_icon")]
    pub loading_icon: Handle<Image>,
    // #[asset(key = "planets", collection(typed, mapped))]
    // planets: HashMap<String, Handle<Image>>,

    // #[asset(key = "lights", collection(typed, mapped))]
    // lights: HashMap<String, Handle<Image>>,

    // #[asset(key = "spheres", collection(typed, mapped))]
    // spheres: HashMap<String, Handle<Image>>,

    // #[asset(key = "noises", collection(typed, mapped))]
    // noises: HashMap<String, Handle<Image>>,

    // #[asset(key = "planet_shaders")]
    // planet_shaders: Handle<EarthlikeShader>
    #[asset(key = "fonts/slkscre.ttf")]
    pub font: Handle<Font>,
}

/// Will be used to load assets when the game starts, so they are all pre-loaded
/// before the game starts.
pub struct GalaxyLoadingPlugin;

impl Plugin for GalaxyLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(EngineState::LoadingAssets).continue_to_state(EngineState::MainMenu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            EngineState::LoadingAssets,
            "core.assets.ron",
        )
        .add_collection_to_loading_state::<_, MyAssets>(EngineState::LoadingAssets)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(EngineState::LoadingAssets), splash_setup)
        .add_systems(
            Update,
            (tick_splash_timer, rotate_loading_icon).run_if(in_state(EngineState::LoadingAssets)),
        )
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(EngineState::LoadingAssets), teardown::<Loaded>);
    }
}

fn setup(mut commands: Commands, query: Query<Entity>) {
    info!("Setting up the world...");

    commands.spawn((
        Camera2dBundle::default(),
        Persist,
        Name::from("Main Camera"),
    ));

    info!("World has been set up!");
}

// a `Timer` for the splash screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Component, Reflect)]
struct LoadingIcon;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //can't use the dynamic asset loader, as is part of the assets being loaded!
    let icon = asset_server.load("sprites/planets/parts/light0.png");

    commands
        .spawn((
            Name::from("Loading Icon Node"),
            Loaded,
            LoadingIcon,
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 20 percent of the screen size, auto adjusting its size accordingly
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

fn rotate_loading_icon(_commands: Commands, mut query: Query<&mut Transform, With<LoadingIcon>>) {
    for mut object in &mut query {
        object.rotate_z(10f32.to_radians());
    }
}

// tick the `SplashTimer` timer
fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.tick(time.delta());
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_rotate_loading_icon() {
        use crate::{
            loading::{rotate_loading_icon, LoadingIcon},
            prelude::*,
        };

        let mut app = App::new();

        app.add_systems(Update, rotate_loading_icon);

        let loading_icon = app
            .world
            .spawn((TransformBundle::default(), LoadingIcon))
            .id();

        app.update();

        assert_eq!(
            app.world.get::<Transform>(loading_icon).unwrap().rotation,
            Quat::from_rotation_z(10f32.to_radians())
        );
    }
}
