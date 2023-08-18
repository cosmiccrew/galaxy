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
                (add_loaded_component/*,planet_rotation,
                planet_randomise */)
                    .run_if(in_state(EngineState::InGame)),
            )
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<MyAssets>,
    // mut materials: ResMut<Assets<EarthlikeMaterial>>,
) {
    let bean_check = check_if_string_eq_bean("bean");
    if !bean_check {
        println!("no bean :(");
    }

    commands.spawn(PlanetBundle {
        planet: Planet {
            seed: 87_654.68,
            pixels: 100,
            rotation: 90f32.to_radians(),
            radius: 100.,
            time_speed: 0.2,
        },

        transform: Transform {
            translation: Vec3::new(100., 100., 0.),
            ..default()
        },
        planet_type: PlanetType::Earthlike(Earthlike {
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
            clouds: Some(Clouds {
                cloud_cover: 0.4,
                ..default()
            }),
            ..default()
        }),
        ..default()
    });

    commands.spawn(PlanetBundle {
        planet_type: PlanetType::Earthlike(Earthlike::default()),
        ..default()
    });

    commands.spawn(PlanetBundle {
        planet: Planet {
            seed: 4.68,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-45., 10., 0.),
            ..default()
        },
        planet_type: PlanetType::Earthlike(Earthlike {
            clouds: Some(Clouds {
                cloud_cover: 0.2,
                ..default()
            }),
            ..default()
        }),
        ..default()
    });
}

// fn planet_rotation(
//     mut commands: Commands,
//     // mut query: Query<&mut Transform, With<Planet>>,
//     mut query: Query<&mut Handle<EarthlikeMaterial>, With<Planet>>,
//     mut materials: ResMut<Assets<EarthlikeMaterial>>,
//     keyboard_input: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let planet_mat: &Handle<EarthlikeMaterial> = query.single();

//     let mut planet_mat = materials.get_mut(planet_mat).unwrap();

//     let mut direction = 0f32;

//     if keyboard_input.pressed(KeyCode::Left) {
//         direction += 1.;
//     }

//     if keyboard_input.pressed(KeyCode::Right) {
//         direction -= 1.;
//     }

//     planet_mat.rotation += (time.delta_seconds() * FRAC_PI_2 * direction);
// }

// fn planet_randomise(
//     mut commands: Commands,
//     mut query: Query<&mut Handle<EarthlikeMaterial>, With<Planet>>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut materials: ResMut<Assets<EarthlikeMaterial>>,
// ) {
//     let planet_mat: &Handle<EarthlikeMaterial> = query.single();

//     let mut planet_mat = materials.get_mut(planet_mat).unwrap();

//     if keyboard_input.just_pressed(KeyCode::Space) {
//         planet_mat.randomise();
//     }

//     let mut direction = 0f32;

//     if keyboard_input.pressed(KeyCode::Up) {
//         direction += 1.;
//     }

//     if keyboard_input.pressed(KeyCode::Down) {
//         direction -= 1.;
//     }

//     planet_mat.pixels += direction;
// }

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
