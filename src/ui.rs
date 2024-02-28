use crate::prelude::*;

pub mod main_menu;
// pub mod settings;

use self::main_menu::GalaxyMainMenuPlugin;

/// Without this plugin, nothing should load. The UI is needed to be able to
/// choose which part of the game you want to enter.
pub struct GalaxyUIPlugin;

impl Plugin for GalaxyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GalaxyMainMenuPlugin);
    }
}
