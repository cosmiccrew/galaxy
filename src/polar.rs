use crate::prelude::*;

pub struct GalaxyPolarPlugin;

impl Plugin for GalaxyPolarPlugin {
    fn build(&self, app: &mut App) {

        // app.add_systems((
        //     setup.in_schedule(OnEnter(EngineState::InGame)),
        //     move_planet.in_set(OnUpdate(EngineState::InGame)),
        //     // game.in_set(OnUpdate(GameState::Game)),
        //     teardown::<Loaded>.in_schedule(OnExit(EngineState::InGame)),
        // ));
    }
}
