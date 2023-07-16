use crate::prelude::*;

pub struct GalaxyWorldPlugin;

impl Plugin for GalaxyWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EngineState::InGame), setup)
            .add_systems(
                Update,
                (planet_rotation, add_loaded_component).run_if(in_state(EngineState::InGame)),
            )
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

#[derive(Component, Reflect)]
struct TempPlanet;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(500., 500.)),
                ..default()
            },
            texture: asset_server.load("planets/planets/planet00.png"),
            ..default()
        },
        TempPlanet,
    ));
}

fn planet_rotation(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<TempPlanet>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut direction = 0f32;

    if keyboard_input.pressed(KeyCode::Left) {
        direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction -= 1.;
    }

    for mut object in &mut query {
        object.rotate_z(direction.to_radians());
    }
}

fn add_loaded_component(
    mut commands: Commands,
    query: Query<Entity, (Without<Loaded>, Without<Persist>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(Loaded);
    }
}
