// .add_plugins(InputManagerPlugin::<Action>::default())

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::prelude::*;

/// This should hold all the information required for players. How they move,
/// interact with the world, are controlled,
pub struct GalaxyInputPlugin;

impl Plugin for GalaxyInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
        // app.add_systems(OnEnter(EngineState::InGame), Self::setup);
    }
}

impl GalaxyInputPlugin {
    fn setup(mut commands: Commands) {}
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
