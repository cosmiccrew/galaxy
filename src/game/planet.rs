use bevy::sprite::MaterialMesh2dBundle;
use leafwing_input_manager::orientation::Orientation;

use crate::prelude::*;

/// This plugin should control how planets function.
pub struct GalaxyPlanetPlugin;

impl Plugin for GalaxyPlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EngineState::InGame), Self::setup)
            .add_systems(
                Update,
                (accelerate_towards_planets, player_adoption, self_right),
            );
    }
}

#[derive(Component, Debug)]
pub struct Planet;

impl GalaxyPlanetPlugin {
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
            PlanetInfluence(500.),
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
            PlanetInfluence(500.),
        ));
    }
}

fn accelerate_towards_planets(
    mut commands: Commands,
    mut player_query: Query<(&mut LinearVelocity, &Mass, &GlobalTransform), With<Player>>,
    mut planet_query: Query<(&Mass, &Children, &GlobalTransform), With<Planet>>,
    time: Res<Time>,
) {
    for (planet_mass, children, planet_transform) in &planet_query {
        for &child in children.iter() {
            let mut child_query = player_query.get_mut(child);

            if let Ok((mut linear_velocity, mass, transform)) = child_query {
                let direction_vector =
                    planet_transform.translation().truncate() - transform.translation().truncate();

                let distance = direction_vector.length();

                let normalised_direction_vector = direction_vector.normalize_or_zero();

                **linear_velocity += normalised_direction_vector
                    * Vec2::splat(
                        ((**mass * **planet_mass) / distance.powi(2)) * 0.1 * time.delta_seconds(),
                    );
            }
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct PlanetInfluence(f32);

impl Default for &PlanetInfluence {
    fn default() -> Self {
        &PlanetInfluence(1000.)
    }
}

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct PlanetNormal(Direction2d);

#[derive(Component, Debug)]
pub struct InSpace;

fn player_adoption(
    mut commands: Commands,
    mut player_query: Query<
        (
            &GlobalTransform,
            &Mass,
            Entity,
            Option<&Parent>,
            &mut ShapeCaster,
        ),
        With<Player>,
    >,
    mut planet_query: Query<
        (&GlobalTransform, &Mass, Entity, Option<&PlanetInfluence>),
        With<Planet>,
    >,
    mut parents_transform: Query<&Transform, With<Planet>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for mut player in &mut player_query {
        let mut player_pos = player.0.translation().truncate();

        let mut closest_planet: Option<(Entity, f32, Vec2)> = None;

        for planet in &planet_query {
            let planet_entity = planet.2;

            let planet_pos = planet.0.translation().truncate();

            //render the planet's influence
            #[cfg(feature = "debug")]
            gizmos.circle_2d(
                planet_pos,
                **planet.3.unwrap_or_default(),
                Color::ALICE_BLUE,
            );

            let distance = player_pos.distance(planet_pos);

            let new_force = G * (**player.1 * **planet.1) / distance.powi(2);

            let direction_vector = planet_pos - player_pos;

            if distance <= **planet.3.unwrap_or_default()
                && new_force
                    > closest_planet
                        .map(|(_, force, _)| force)
                        .unwrap_or(f32::MIN)
            {
                closest_planet = Some((planet_entity, new_force, direction_vector));
            }
        }

        if let Some(closest_planet) = closest_planet {
            // This won't possibly cause issues...
            let direction2d = Direction2d::new(closest_planet.2).unwrap_or(Direction2d::X);

            commands.entity(player.2).insert(PlanetNormal(direction2d));

            if !player.3.is_some_and(|x| x.get() == closest_planet.0) {
                commands
                    .entity(player.2)
                    .set_parent_in_place(closest_planet.0)
                    .remove::<InSpace>();
            }
        } else {
            commands
                .entity(player.2)
                .remove_parent_in_place()
                .remove::<PlanetNormal>()
                .insert(InSpace);
        }
    }
}

fn self_right(
    mut commands: Commands,
    mut player_query: Query<
        (
            &Rotation,
            &mut AngularVelocity,
            // &RayHits,
            &Parent,
            &PlanetNormal,
            &Transform,
        ),
        (With<Player>, With<Parent>),
    >,
    mut planet_query: Query<&Rotation, With<Planet>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for (rotation, mut angular_velocity, parent, planet_normal, transform) in &mut player_query {
        let parent = parent.get();

        /// SAFETY - this query is `Has<Planet>`, so is a bool guarenteed.
        if let Ok(planet_rotation) = planet_query.get(parent) {
            let planet_tangent = planet_normal.perp();

            let planet_tangent_angle = planet_tangent.to_angle();

            let player_rotation = rotation.as_radians();

            let mut diff_rotation = planet_tangent_angle - player_rotation;

            if diff_rotation > PI {
                diff_rotation -= TAU
            } else if diff_rotation < -PI {
                diff_rotation += TAU
            }

            if !(diff_rotation < FRAC_PI_8 / 2f32 && diff_rotation > -FRAC_PI_8 / 2f32) {
                angular_velocity.0 += diff_rotation / PI;
            }

            // if let Some(&floor_vector) = hits.iter().next().filter(|ray|
            // ray.entity == parent ) {     let normal_angle =
            // (-**local_down).angle_between(floor_vector.normal);
            //     if normal_angle <= FRAC_PI_8 && normal_angle >= -FRAC_PI_8 {
            //     angular_velocity.0 += (normal_angle/FRAC_PI_2);
            //     }
            // }
        }
    }
}
