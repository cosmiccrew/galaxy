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
            //disables the inbuilt gravity
            .insert_resource(Gravity::ZERO);
    }
}

#[derive(Component, Debug, Default)]
pub struct Grounded;
