use bevy::{
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::prelude::*;

pub struct GalaxyGamePlugin;

impl Plugin for GalaxyGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EngineState::InGame), setup)
            .add_systems(
                Update,
                (
                    add_loaded_component,
                    planet_rotation,
                    planet_randomise,
                    planet_change_pixels,
                )
                    .run_if(in_state(EngineState::InGame)),
            )
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<MyAssets>,
    mut earthlike_materials: ResMut<Assets<Earthlike>>,
    mut cloud_cover_materials: ResMut<Assets<CloudCover>>,
) {
    let bean_check = check_if_string_eq_bean("bean");
    if !bean_check {
        println!("no bean :(");
    }

    commands.spawn((
        CelestialBundle {
            transform: Transform {
                translation: Vec3::new(350., 200., 0.),
                ..default()
            },
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(200., 200.)).into())
                .into(),
            celestial_shader: earthlike_materials.add(Earthlike {
                celestial: CelestialSettings {
                    seed: 87_654.68,
                    pixels: 100.,
                    rotation: 90f32.to_radians(),
                    radius: 100.,
                    time_speed: 10.,
                },
                land_colours: [
                    Color::rgb(0.388235, 0.670588, 0.247059),
                    Color::rgb(0.231373, 0.490196, 0.309804),
                    Color::rgb(0.184314, 0.341176, 0.32549),
                    Color::rgb(0.156863, 0.207843, 0.25098),
                ],
                river_colours: [
                    Color::rgb(0.184314, 0.341176, 0.32549),
                    Color::rgb(0.156863, 0.207843, 0.25098),
                ],
                ..default()
            }),
            ..default()
        },
        // cloud_cover_materials.add(CloudCover {
        //     cloud_cover: 0.4,
        //     ..default()
        // }),
    ));

    commands.spawn(CelestialBundle {
        transform: Transform::from_xyz(-450., -100., 0.),
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(300., 300.)).into())
            .into(),
        celestial_shader: earthlike_materials.add(Earthlike::default()),
        ..default()
    });

    commands.spawn((
        CelestialBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(500., 500.)).into())
                .into(),
            celestial_shader: earthlike_materials.add(Earthlike {
                celestial: CelestialSettings {
                    seed: 4.68,
                    ..default()
                },

                ..default()
            }),
            ..default()
        },
        // cloud_cover_materials.add(CloudCover {
        //     cloud_cover: 0.2,
        //     ..default()
        // }),
    ));
}

fn planet_rotation(
    mut commands: Commands,
    // mut query: Query<&mut Transform, With<Planet>>,
    mut query: Query<&mut Handle<Earthlike>, With<Celestial>>,
    mut materials: ResMut<Assets<Earthlike>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for earthlike_handle in query.iter() {
        let mut earthlike_material = materials.get_mut(earthlike_handle).unwrap();

        let mut direction = 0f32;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += 1.;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction -= 1.;
        }

        earthlike_material.celestial.rotation += (time.delta_seconds() * FRAC_PI_2 * direction);
    }
}

fn planet_randomise(
    mut commands: Commands,
    mut query: Query<&mut Handle<Earthlike>, With<Celestial>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<Earthlike>>,
) {
    for earthlike_handle in query.iter() {
        let mut earthlike_material = materials.get_mut(earthlike_handle).unwrap();

        if keyboard_input.just_pressed(KeyCode::Space) {
            earthlike_material.randomise();
        }
    }
}

fn planet_change_pixels(
    mut commands: Commands,
    // mut query: Query<&mut Transform, With<Planet>>,
    mut query: Query<&mut Handle<Earthlike>, With<Celestial>>,
    mut materials: ResMut<Assets<Earthlike>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for earthlike_handle in query.iter() {
        let mut earthlike_material = materials.get_mut(earthlike_handle).unwrap();

        let mut direction = 0f32;

        if keyboard_input.pressed(KeyCode::Up) {
            direction += 1.;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= 1.;
        }

        earthlike_material.celestial.pixels += direction;
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_adding_loaded_component() {
        use crate::game::add_loaded_component;
        use crate::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, add_loaded_component);

        let should_have = app
            .world
            .spawn(Name::new("Should have loaded component"))
            .id();
        let should_not_change = app
            .world
            .spawn((Persist, Name::new("Should not change")))
            .id();

        app.update();

        assert!(app.world.get::<Loaded>(should_have).is_some());
        assert!(app.world.get::<Loaded>(should_not_change).is_none());
    }
}
