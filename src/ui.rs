use crate::prelude::*;

pub mod main_menu;

use self::main_menu::GalaxyMainMenuPlugin;

pub struct GalaxyUIPlugin;

impl Plugin for GalaxyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GalaxyMainMenuPlugin,));
        // app.add_systems(OnEnter(EngineState::InGame), setup)
        //     .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}
