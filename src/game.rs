pub mod planet;

use bevy::sprite::MaterialMesh2dBundle;

use self::planet::GalaxyPlanetPlugin;
use crate::prelude::*;

/// This plugin should control the rendering of the entire world.
pub struct GalaxyGamePlugin;

impl Plugin for GalaxyGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(WinitSettings::default())
            .add_plugins(GalaxyPlanetPlugin)
            .add_systems(OnEnter(EngineState::InGame), Self::setup)
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

#[derive(Component, Debug)]
struct Planet;

impl GalaxyGamePlugin {
    fn setup() {}
}
