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
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    asset_folder: {
                        if cfg!(all(
                            target_os = "macos",
                            not(debug_assertions),
                            not(features = "dynamic_linking")
                        )) {
                            "../Resources/assets".to_string()
                        } else {
                            "assets".to_string()
                        }
                    },
                })
                .set({
                    use bevy::log::LogPlugin;
                    if cfg!(debug_assertions) {
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

pub fn check_if_string_eq_bean(string: &str) -> bool {
    string == "bean"
}

#[test]
fn test_check_if_string_eq_bean() {
    assert!(check_if_string_eq_bean("bean"));

    assert!(!check_if_string_eq_bean("not bean"));
}
