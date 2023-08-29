use crate::prelude::*;

pub struct GalaxyMainMenuPlugin;

impl Plugin for GalaxyMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(OnEnter(EngineState::MainMenu), setup)
            .add_systems(Update, main_button_system)
            .add_systems(OnExit(EngineState::MainMenu), teardown::<Loaded>);
    }
}

const ENABLED_BUTTON: Color = Color::WHITE;
const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const DISABLED_BUTTON: Color = Color::DARK_GRAY;

const ENABLED_BUTTON_BORDER: Color = Color::BLACK;
const DISABLED_BUTTON_BORDER: Color = Color::GRAY;

#[derive(Component, Debug, Default, Reflect, PartialEq, Eq, Clone, Copy)]
#[reflect(Component)]
enum ButtonState {
    #[default]
    Enabled,
    Disabled,
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
enum MainButtonType {
    Local,
    Online,
    Settings,
}

#[derive(Component, Debug, Default, Reflect, PartialEq, Eq, Clone, Copy)]
#[reflect(Component)]
struct MainMenuButton;

fn main_menu_button(parent: &mut ChildBuilder, button_type: MainButtonType, state: ButtonState) {
    let text = match button_type {
        MainButtonType::Local => "Local (1-4)",
        MainButtonType::Online => "Online (1-8)",
        MainButtonType::Settings => "Settings",
    };

    parent
        .spawn((
            state,
            button_type,
            Name::from(format!("Main Menu Button ({text})")),
            ButtonBundle {
                style: Style {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Percent(100.),
                    height: Val::Percent(20.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                // image: ,
                background_color: match state {
                    ButtonState::Disabled => DISABLED_BUTTON,
                    ButtonState::Enabled => ENABLED_BUTTON,
                }
                .into(),
                border_color: match state {
                    ButtonState::Enabled => ENABLED_BUTTON_BORDER,
                    ButtonState::Disabled => DISABLED_BUTTON_BORDER,
                }
                .into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.4, 0.4, 0.4),
                    ..default()
                },
            ));
        });
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
                    padding: UiRect::vertical(Val::Percent(5.)),
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
                        padding: UiRect::vertical(Val::Percent(5.)),
                        width: Val::Percent(50.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column, //align items in a column
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,

                        ..default()
                    },
                    border_color: Color::GREEN.into(),
                    ..default()
                })
                .with_children(|parent| {
                    main_menu_button(parent, MainButtonType::Local, ButtonState::Enabled);
                    main_menu_button(parent, MainButtonType::Online, ButtonState::Disabled);
                    main_menu_button(parent, MainButtonType::Settings, ButtonState::Disabled);
                });
        });
}

fn main_button_system(
    mut engine_state: ResMut<NextState<EngineState>>,
    // mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut MainButtonType,
            &mut ButtonState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, main_button_type, button_state) in
        &mut interaction_query
    {
        if *button_state == ButtonState::Enabled {
            match *interaction {
                Interaction::Pressed => {
                    match *main_button_type {
                        MainButtonType::Local => {
                            engine_state.set(EngineState::InGame);
                        }
                        MainButtonType::Online => {
                            // engine_state.set(EngineState::InGame);
                        }
                        MainButtonType::Settings => {
                            // engine_state.set(EngineState::InGame);
                        }
                    }

                    // border_color.0 = Color::RED;
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::DARK_GRAY;
                }
                Interaction::None => {
                    *color = ENABLED_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}

// fn every_other_time() -> impl Condition<()> {
//     IntoSystem::into_system(|mut flag: Local<bool>| {
//         if *flag {
//             true
//         } else {
//             *flag = true;
//             false
//         }
//     })
// }

#[cfg(test)]
mod tests {

    // use crate::{
    //     prelude::*,
    //     ui::main_menu::{main_button_system, main_menu_button, setup},
    // };

    // #[test]
    // fn test_main_button_system() {
    //     // Setup app
    //     let mut app = App::new();

    //     // let clickable = app
    //     //     .world
    //     //     .spawn(NodeBundle::default())
    //     //     .with_children(|parent| {

    //     //         main_menu_button(parent, "test_button", true)
    //     //     });

    //     // let unclickable = app
    //     //     .world
    //     //     .spawn(NodeBundle::default())
    //     //     .with_children(|parent| {

    //     //         main_menu_button(parent, "test_button", true)
    //     //     });

    //     // Add our systems
    //     app.add_plugins(MinimalPlugins)
    //         .add_systems(Startup, setup)
    //         .add_systems(Update, main_button_system);

    //     // app.world.query();

    //     // Run systems
    //     app.update();

    //     // assert_eq!(app.world.query::<&Enemy>().iter(&app.world).len(), 1);

    //     // Run systems
    //     app.update();

    //     // assert_eq!(app.world.query::<&Enemy>().iter(&app.world).len(), 1);
    // }
}
