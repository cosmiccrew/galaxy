use crate::prelude::*;

pub struct GalaxySettingsMenuPlugin;

impl Plugin for GalaxySettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MainMenuState::Settings), setup)
            // .add_systems(Update, main_menu_button_system)
        ;
    }
}

// // One of the two settings that can be set through the menu. It will be a
// resource in the app #[derive(Resource, Debug, Component, PartialEq, Eq,
// Clone, Copy)] enum DisplayQuality {
//     Low,
//     Medium,
//     High,
// }

// One of the two settings that can be set through the menu. It will be a
// resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u8);

fn setup() {}
