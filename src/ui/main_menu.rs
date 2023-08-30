use crate::prelude::*;

pub struct GalaxyMainMenuPlugin;

impl Plugin for GalaxyMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .insert_resource(WinitSettings::desktop_app())
            .add_systems(OnEnter(EngineState::MainMenu), setup)
            .add_systems(Update, main_menu_button_system)
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
enum MainMenuButton {
    Local,
    Online,
    Settings,
}

fn setup(mut commands: Commands, assets: Res<MyAssets>) {
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
                .spawn((
                    Name::from("Menu Buttons"),
                    NodeBundle {
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
                    },
                ))
                .with_children(|parent| {
                    let font = assets.font.clone();

                    let mut main_menu_button = |button_type: MainMenuButton, state: ButtonState| {
                        let text = match button_type {
                            MainMenuButton::Local => "Local (1-4)",
                            MainMenuButton::Online => "Online (1-8)",
                            MainMenuButton::Settings => "Settings",
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
                                parent.spawn((
                                    Name::from("Text Bundle"),
                                    TextBundle::from_section(
                                        text,
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 40.0,
                                            color: Color::rgb(0.4, 0.4, 0.4),
                                            ..default()
                                        },
                                    ),
                                ));
                            });
                    };

                    main_menu_button(MainMenuButton::Local, ButtonState::Enabled);
                    main_menu_button(MainMenuButton::Online, ButtonState::Disabled);
                    main_menu_button(MainMenuButton::Settings, ButtonState::Disabled);
                });
        });
}

fn main_menu_button_system(
    mut engine_state: ResMut<NextState<EngineState>>,
    // mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut MainMenuButton,
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
                        MainMenuButton::Local => {
                            //in future, this should go to another main menu section, where
                            engine_state.set(EngineState::InGame);
                        }
                        MainMenuButton::Online => {
                            todo!();
                            // engine_state.set(EngineState::InGame);
                        }
                        MainMenuButton::Settings => {
                            todo!();
                            // engine_state.set(EngineState::InGame);
                        }
                    }
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

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    use super::{main_menu_button_system, ButtonState, MainMenuButton};

    #[test]
    fn test_main_menu_button_system() {
        // Setup app
        let mut app = App::new();

        app.add_state::<EngineState>()
            .add_systems(Update, main_menu_button_system);

        let enabled = app
            .world
            .spawn((
                Button,
                ButtonState::Enabled,
                MainMenuButton::Local,
                Name::from("Enabled"),
                Interaction::default(),
                BackgroundColor::default(),
                BorderColor::default(),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::default());
            })
            .id();

        let disabled = app
            .world
            .spawn((
                Button,
                ButtonState::Disabled,
                MainMenuButton::Local,
                Name::from("Disabled"),
                Interaction::default(),
                BackgroundColor::default(),
                BorderColor::default(),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::default());
            })
            .id();

        app.update();

        // nothing is clicked, so state should be the default
        assert_eq!(
            app.world
                .get_resource::<State<EngineState>>()
                .unwrap()
                .get(),
            &EngineState::default()
        );

        //press the disabled button
        *app.world.get_mut::<Interaction>(disabled).unwrap() = Interaction::Pressed;

        //has to be performed twice, as state changes take two updates
        app.update();
        app.update();

        //as this button is disabled, nothing should change here
        assert_eq!(
            app.world
                .get_resource::<State<EngineState>>()
                .unwrap()
                .get(),
            &EngineState::default()
        );

        app.update();
        app.update();

        //press the enabled button
        *app.world.get_mut::<Interaction>(enabled).unwrap() = Interaction::Pressed;

        //has to be performed twice, as state changes take two updates
        app.update();
        app.update();

        //the state should have now changed!
        assert_ne!(
            app.world
                .get_resource::<State<EngineState>>()
                .unwrap()
                .get(),
            &EngineState::default()
        );
    }

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
