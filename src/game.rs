use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;

/// This plugin should control the rendering of the entire world.
pub struct GalaxyGamePlugin;

impl Plugin for GalaxyGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(WinitSettings::default())
            .add_systems(OnEnter(EngineState::InGame), Self::setup)
            .add_systems(Update, accelerate_towards_planets)
            //     (planet_rotation, planet_randomise, planet_change_pixels)
            //         .run_if(in_state(EngineState::InGame)),
            // )
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

#[derive(Component, Debug)]
struct Planet;

impl GalaxyGamePlugin {
    fn setup(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle { radius: 100.0 }).into(),
                material: materials.add(Color::GREEN),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            Name::new("Planet"),
            Collider::circle(100.),
            RigidBody::Static,
            Planet,
        ));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle { radius: 100.0 }).into(),
                material: materials.add(Color::GREEN),
                transform: Transform::from_xyz(400., 150., 0.),
                ..default()
            },
            Name::new("Planet"),
            Collider::circle(100.),
            RigidBody::Static,
            Planet,
        ));
    }
}

fn accelerate_towards_planets(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut LinearVelocity, &Mass), With<Player>>,
    mut planet_query: Query<(&Transform, &Mass), With<Planet>>,
    time: Res<Time>,
) {
    for (player_transform, mut linear_velocity, player_mass) in &mut player_query {
        // let mut direction = Vec2::ZERO;

        let player_pos = player_transform.translation.truncate();

        for planet in &planet_query {
            //  direction += planet.translation;

            let planet_translation = planet.0.translation.truncate();

            let distance = player_pos.distance(planet_translation);

            if distance <= 1000. {
                let direction_vector = planet_translation - player_pos;

                let normalised_direction_vector = direction_vector.normalize_or_zero();

                **linear_velocity += normalised_direction_vector
                    * Vec2::splat(
                        (**player_mass * **planet.1 / distance.powi(2))
                            * 0.1
                            * time.delta_seconds(),
                    );
            }

            // info!("{direction:?}");
        }
    }
}

// fn distance_to_planets(
//     mut commands: Commands,
//     mut player_query: Query<(&Transform, &LinearVelocity, &mut Entity),
// With<Player>>,     mut planet_query: Query<(&Transform), With<Planet>>,
//     time: Res<Time>,
// ) {
//     let planet_distance = PlanetDistance(vec![]);

//     for (player_transform, linear_velocity, mut entity) in &mut player_query
// {         let player_pos = player_transform.translation.truncate();

//         for planet_transform in &planet_query {
//             //  direction += planet.translation;

//             let planet_translation = planet_transform.translation.truncate();

//             let distance = player_pos.distance(planet_translation);

//             if distance <= 1000. {
//                 let direction_vector = planet_translation - player_pos;

//                 let normalised_direction_vector =
// direction_vector.normalize_or_zero();

//                 planet_distance * *linear_velocity +=
// normalised_direction_vector
//                     * Vec2::splat( (**player_mass * **planet.1 /
//                       distance.powi(2))
//                             * 0.1
//                             * time.delta_seconds(),
//                     );
//             }
//         }
//     }
// }

// struct PlanetDistance(Vec<(Vec2, Entity)>);
