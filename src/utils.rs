use std::any::type_name;

use crate::prelude::*;

pub struct GalaxyDefaultPlugins {
    pub log_level: bevy::log::Level,
}

impl Plugin for GalaxyDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::DARK_GRAY))
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Cosmic Crew: Galaxy".to_string(),
                            prevent_default_event_handling: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .set({
                        use bevy::log::{Level, LogPlugin};

                        let filter = match self.log_level {
                            Level::ERROR => "error",
                            Level::WARN => "warn",
                            Level::INFO => "info",
                            Level::DEBUG => "debug",
                            Level::TRACE => "trace,bevy_xpbd_2d=info",
                        }
                        .to_string()
                            + ",wgpu_core=warn,wgpu_hal=warn,wgpu=error,naga=warn";

                        LogPlugin {
                            filter,
                            level: self.log_level,
                            ..default()
                        }
                    })
                    .set(ImagePlugin::default_nearest()),
            );
    }
}

#[derive(Component, Reflect)]
pub struct Persist;

#[derive(Component, Reflect)]
pub struct Loaded;

///generic system that takes a component as a parameter, and will despawn
/// (teardown) all entities with that component, that don't have the component
/// [Persist]
pub fn teardown<T: Component>(
    to_despawn: Query<(Entity, Option<&Name>), (With<T>, Without<Persist>)>,
    mut commands: Commands,
) {
    debug!(
        "Tearing down entities with component: \"{}\"",
        type_name::<T>()
    );

    for (entity, name) in &to_despawn {
        commands.entity(entity).despawn_recursive();
        if let Some(name) = name {
            debug!("despawned: \"{name}\"");
        } else {
            debug!("despawned: \"{entity:?}\"");
        }
    }
}

///probably dangerous to use!
#[deprecated]
pub fn add_loaded_component(
    mut commands: Commands,
    query: Query<Entity, (Without<Loaded>, Without<Persist>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(Loaded);
    }
}

#[allow(deprecated)]
#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn test_despawn_components() {
        let mut app = App::new();

        app.add_systems(Update, teardown::<Loaded>);

        let should_despawn = app.world.spawn(Loaded).id();

        let should_persist = app.world.spawn((Persist, Loaded)).id();

        app.update();

        assert!(app.world.get::<Loaded>(should_despawn).is_none());
        assert!(app.world.get::<Loaded>(should_persist).is_some());
    }

    #[test]
    fn test_adding_loaded_component() {
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
