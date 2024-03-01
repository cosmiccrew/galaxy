// .add_plugins(InputManagerPlugin::<Action>::default())

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::{player::PlayerJump, prelude::*};

/// This should hold all the information required for players. How they move,
/// interact with the world, are controlled,
pub struct GalaxyInputPlugin;

impl Plugin for GalaxyInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(
                Update,
                Self::player_input_sender.run_if(in_state(EngineState::InGame)),
            );
        // app.add_systems(OnEnter(EngineState::InGame), Self::setup);
    }
}

impl GalaxyInputPlugin {
    fn setup(mut commands: Commands) {}

    pub fn player_input_sender(
        mut commands: Commands,
        mut actions: Query<&ActionState<Action>, With<Player>>,
        mut movement_writer: EventWriter<PlayerMovement>,
        mut jump_writer: EventWriter<PlayerJump>,
    ) {
        let action_state = actions.single();

        let mut overall_direction = Vec2::ZERO;

        Action::DIRECTIONS.into_iter().for_each(|direction_action| {
            if action_state.pressed(&direction_action) {
                if let Some(direction) = direction_action.direction() {
                    overall_direction += *direction;
                }
            }
        });

        let overall_direction = Direction2d::new(overall_direction);

        if let Ok(direction) = overall_direction {
            movement_writer.send(PlayerMovement { direction });
        }

        if action_state.pressed(&Action::Jump) {
            jump_writer.send(PlayerJump);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
    Jump,
}

impl Action {
    pub const DIRECTIONS: [Action; 4] = [Action::Left, Action::Down, Action::Right, Action::Up];

    pub fn direction(self) -> Option<Direction2d> {
        match self {
            Action::Up => Some(Direction2d::Y),
            Action::Down => Some(Direction2d::NEG_Y),
            Action::Left => Some(Direction2d::NEG_X),
            Action::Right => Some(Direction2d::X),
            _ => None,
        }
    }
}
