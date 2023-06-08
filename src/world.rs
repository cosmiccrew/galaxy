use crate::prelude::*;

pub struct GalaxyWorldPlugin;

#[derive(Component)]
pub struct Loaded;

impl Plugin for GalaxyWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            setup.in_schedule(OnEnter(EngineState::InGame)),
            // game.in_set(OnUpdate(GameState::Game)),
            teardown::<Loaded>.in_schedule(OnExit(EngineState::InGame)),
        ));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(500., 500.)),
                ..default()
            },
            texture: asset_server.load("planets/planets/planet00.png"),
            ..default()
        },
        Loaded,
    ));
}
