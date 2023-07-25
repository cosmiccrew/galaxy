#![allow(unused)]
#![allow(clippy::type_complexity)]

pub mod debug;
pub mod game;
pub mod loading;
pub mod player;
pub mod polar;
pub mod state;
pub mod utils;

pub mod prelude {

    pub use std::f32::consts::*;

    pub use anyhow::{anyhow, bail, ensure, Result};
    pub use bevy::prelude::*;

    pub use crate::debug::*;
    pub use crate::game::*;
    pub use crate::loading::*;
    pub use crate::player::*;
    pub use crate::polar::*;
    pub use crate::state::*;
    pub use crate::utils::*;
    pub use crate::{PLANETS, PLANET_PARTS};
}

pub const PLANETS: &str = "planets/planets";
pub const PLANET_PARTS: &str = "planets/parts";
