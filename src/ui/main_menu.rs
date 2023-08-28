use crate::prelude::*;

pub struct GalaxyMainMenuPlugin;

impl Plugin for GalaxyMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(OnEnter(EngineState::MainMenu), setup)
            .add_systems(OnExit(EngineState::MainMenu), teardown::<Loaded>);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Main UI Node
    commands
        .spawn((
            Name::from("Main UI Node"),
            Loaded,
            NodeBundle {
                style: Style {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: Color::RED.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            //Menu Buttons (50% center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(5.)),
                        width: Val::Percent(50.),
                        height: Val::Percent(100.),

                        // position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    border_color: Color::GREEN.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(5.)),
                                width: Val::Percent(100.),
                                height: Val::Percent(20.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            // image: ,
                            border_color: Color::BLACK.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Local (1-4)",
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}
