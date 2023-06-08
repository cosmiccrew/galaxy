use crate::prelude::*;

///generic system that takes a component as a parameter, and will despawn
/// (teardown) all entities with that component
pub fn teardown<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
