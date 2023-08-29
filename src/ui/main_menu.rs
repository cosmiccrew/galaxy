use crate::prelude::*;

pub struct GalaxyMainMenuPlugin;

impl Plugin for GalaxyMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
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

// #[derive(Component, Debug, Default, Reflect, PartialEq, Eq, Clone, Copy)]
// #[reflect(Component)]
// struct MainMenuButton;

fn main_menu_button(parent: &mut ChildBuilder, button_type: MainMenuButton, state: ButtonState) {
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
                    main_menu_button(parent, MainMenuButton::Local, ButtonState::Enabled);
                    main_menu_button(parent, MainMenuButton::Online, ButtonState::Disabled);
                    main_menu_button(parent, MainMenuButton::Settings, ButtonState::Disabled);
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
    println!("f");
    for (interaction, mut color, mut border_color, main_button_type, button_state) in
        &mut interaction_query
    {
        println!("g");
        if *button_state == ButtonState::Enabled {
            println!("e");
            match *interaction {
                Interaction::Pressed => {
                    match *main_button_type {
                        MainMenuButton::Local => {
                            engine_state.set(EngineState::InGame);
                        }
                        MainMenuButton::Online => {
                            // engine_state.set(EngineState::InGame);
                        }
                        MainMenuButton::Settings => {
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

        let mut disabled_interaction = app.world.get_mut::<Interaction>(disabled).unwrap();

        *disabled_interaction = Interaction::Pressed;

        //has to be performed twice, as state changes take two updates
        app.update();
        app.update();

        //as this button is disabled, nothing should change
        assert_eq!(
            app.world
                .get_resource::<State<EngineState>>()
                .unwrap()
                .get(),
            &EngineState::default()
        );

        let mut enabled_interaction = app.world.get_mut::<Interaction>(enabled).unwrap();

        println!("{:?}", &*enabled_interaction);

        *enabled_interaction = Interaction::Pressed;

        println!("{:?}", &*enabled_interaction);

        //has to be performed twice, as state changes take two updates
        app.update();
        app.update();

        //and here the state should have changed
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
