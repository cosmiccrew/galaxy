use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;

/// This plugin should control how planets function.
pub struct GalaxyPlanetPlugin;

impl Plugin for GalaxyPlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EngineState::InGame), Self::setup)
            .add_systems(Update, (accelerate_towards_planets, player_adoption));
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

#[derive(Component, Deref)]
struct PlanetInfluence(f32);

impl Default for &PlanetInfluence {
    fn default() -> Self {
        &PlanetInfluence(1000.)
    }
}

fn player_adoption(
    mut commands: Commands,
    mut player_query: Query<(&GlobalTransform, &Mass, Entity, Option<&Parent>), With<Player>>,
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

            if distance <= **planet.3.unwrap_or_default()
                && new_force
                    > closest_planet
                        .map(|(_, force, _)| force)
                        .unwrap_or(f32::MIN)
            {
                closest_planet = Some((planet_entity, new_force, (planet_pos - player_pos)));
            }
        }

        if let Some(closest_planet) = closest_planet {
            if !player.3.is_some_and(|x| x.get() == closest_planet.0) {
                commands
                    .entity(player.2)
                    .set_parent_in_place(closest_planet.0);
            }
        } else {
            commands.entity(player.2).remove_parent_in_place();
        }
    }
}
