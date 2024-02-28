use bevy_xpbd_2d::plugins::PhysicsPlugins;

use crate::prelude::*;

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
