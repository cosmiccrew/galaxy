use std::any::type_name;
use std::time::Duration;

use bevy::asset::ChangeWatcher;

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

pub struct GalaxyDefaultPlugins;

impl Plugin for GalaxyDefaultPlugins {
    fn build(&self, mut app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK)).add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cosmic Crew: Galaxy".to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    asset_folder: {
                        if cfg!(all(
                            target_os = "macos",
                            not(feature = "debug"),
                            not(feature = "fast_compile"),
                        )) {
                            "../Resources/assets".to_string()
                        } else {
                            "assets".to_string()
                        }
                    },
                })
                .set({
                    use bevy::log::LogPlugin;
                    if cfg!(feature = "debug") {
                        LogPlugin {
                            level: bevy::log::Level::DEBUG,
                            filter: "debug,wgpu_core=warn,wgpu_hal=warn,naga=info,bevy=info".into(),
                        }
                    } else {
                        // this code is compiled only if debug assertions are disabled (release
                        // mode)
                        LogPlugin {
                            level: bevy::log::Level::INFO,
                            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                        }
                    }
                })
                .set(ImagePlugin::default_nearest()),
        );
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_despawn_components() {
        use crate::prelude::*;

        let mut app = App::new();

        app.add_systems(Update, teardown::<Loaded>);

        let should_despawn = app.world.spawn(Loaded).id();

        let should_persist = app.world.spawn((Persist, Loaded)).id();

        app.update();

        assert!(app.world.get::<Loaded>(should_despawn).is_none());
        assert!(app.world.get::<Loaded>(should_persist).is_some());
    }
}
