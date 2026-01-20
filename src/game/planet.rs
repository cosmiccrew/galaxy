use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
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

#[derive(Component, Deref, DerefMut)]
struct Radius(f32);

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub influence: PlanetInfluence,
}

impl PlanetBundle {
    pub fn new(
        material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
        collider: Collider,
        rigid_body: RigidBody,
        influence: PlanetInfluence,
    ) -> Self {
        Self {
            material_mesh_bundle,
            collider,
            rigid_body,
            influence,
        }
    }
}

#[derive(Component, Debug)]
pub struct Planet;

// #[derive(Asset)]
// struct PlanetMesh;

impl GalaxyPlanetPlugin {
    fn setup(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let planet_texture = assets.load("sprites/planets/planets/planet00.png");

        commands.spawn((
            // MaterialMesh2dBundle {
            //     mesh: meshes.add(Circle { radius: 1000.0 }).into(),
            //     material: materials.add(Color::GREEN),
            //     transform: Transform::from_xyz(0., 0., 0.),
            //     ..default()
            // },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2500., 2500.)),
                    ..default()
                },
                texture: planet_texture.clone(),
                transform: Transform::from_xyz(0., 0., 0.),

                ..default()
            },
            Name::new("Planet"),
            Collider::circle(1000.),
            RigidBody::Static,
            Planet,
            PlanetInfluence(5000.), // PlanetBundle { influence: PlanetInfluence(500.) },
        ));

        commands.spawn((
            // MaterialMesh2dBundle {
            //     mesh: meshes.add(Circle { radius: 1000.0 }).into(),
            //     material: materials.add(Color::GREEN),
            //     transform: Transform::from_xyz(2000., 2500., 0.),
            //     ..default()
            // },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2000., 2000.)),
                    ..default()
                },
                texture: planet_texture.clone(),
                transform: Transform::from_xyz(2000., 2500., 0.),

                ..default()
            },
            Name::new("Planet"),
            Collider::circle(1000.),
            RigidBody::Static,
            PlanetInfluence(5000.),
            Planet, // PlanetBundle { influence: PlanetInfluence(500.) },
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

#[derive(Component, Deref, DerefMut, Debug, Default, Clone, Copy)]
pub struct PlanetInfluence(f32);

impl Default for &PlanetInfluence {
    fn default() -> Self {
        &PlanetInfluence(500.)
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
            if let Ok(direction2d) = Direction2d::new(closest_planet.2) {
                commands.entity(player.2).insert(PlanetNormal(direction2d));
            }

            if player.3.is_none_or(|x| x.get() != closest_planet.0) {
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
        (&mut Transform, &Parent, &PlanetNormal),
        (With<Player>, With<Parent>, Without<Planet>),
    >,
    mut planet_query: Query<&Planet>,
    time: Res<Time>,
) {
    for (mut player_transform, parent, planet_normal) in &mut player_query {
        let parent = parent.get();

        if planet_query.get(parent).is_ok() {
            let planet_tangent = planet_normal.perp();

            let planet_tangent_angle = Quat::from_rotation_z(planet_tangent.to_angle());

            let rotation_lerp: Quat = player_transform.rotation.lerp(planet_tangent_angle, 0.25);

            player_transform.rotation = rotation_lerp;
        }
    }
}
