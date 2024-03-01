use bevy_xpbd_2d::plugins::PhysicsPlugins;

use crate::prelude::*;

// #[allow(non_upper_case_globals)]
/// The gravitational constant G - note this is not g, the gravity of earth,
/// however in this pixel world I have made it also 9.81 because... yes.
pub const G: f32 = 9.81;

/// Used to begin and enable the underlying physics engine.
pub struct GalaxyPhysicsPlugin;

impl Plugin for GalaxyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
            .add_systems(
                Update,
                update_grounded.run_if(in_state(EngineState::InGame)),
            )
            //disables the inbuilt gravity
            .insert_resource(Gravity::ZERO);
    }
}

#[derive(Component, Debug, Default)]
#[component(storage = "SparseSet")]
pub struct Grounded;

fn update_grounded(mut commands: Commands, mut query: Query<(Entity, &ShapeHits), With<Player>>) {
    for (entity, hits) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        let is_grounded = hits.iter().any(|_hit| true);

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}
