use crate::prelude::*;

pub struct GalaxyMainMenuPlugin;

impl Plugin for GalaxyMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(OnEnter(EngineState::MainMenu), setup)
            .add_systems(PreUpdate, add_loaded_component)
            .add_systems(OnExit(EngineState::MainMenu), teardown::<Loaded>);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });
}
