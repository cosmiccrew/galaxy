#![allow(unused)]
#![allow(clippy::type_complexity)]
#![warn(dead_code)]

pub mod debug;
pub mod game;
pub mod loading;
pub mod player;
pub mod polar;
pub mod shaders;
pub mod state;
pub mod utils;

pub mod prelude {

    pub use std::f32::consts::*;

    pub use bevy::{prelude::*, reflect::*};

    pub use anyhow::{anyhow, bail, ensure, Result};
    pub use rand::prelude::*;

    pub use crate::debug::*;
    pub use crate::game::*;
    pub use crate::loading::*;
    pub use crate::player::*;
    pub use crate::polar::*;
    pub use crate::shaders::*;
    pub use crate::state::*;
    pub use crate::utils::*;
}
