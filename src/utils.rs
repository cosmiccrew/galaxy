use std::any::type_name;

use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct Persist;

#[derive(Component, Reflect)]
pub struct Loaded;

///generic system that takes a component as a parameter, and will despawn
/// (teardown) all entities with that component, that don't have the component
/// [Persist]
pub fn teardown<T: Component>(
    to_despawn: Query<Entity, (With<T>, Without<Persist>)>,
    mut commands: Commands,
) {
    debug!(
        "Tearing down entities with component: \"{}\"",
        type_name::<T>()
    );

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
