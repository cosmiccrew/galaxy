// #![allow(unused)]
#![allow(clippy::type_complexity, clippy::needless_update)]
// #![warn(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "debug")]
pub mod debug;
pub mod game;
pub mod loading;
pub mod player;
pub mod polar;
pub mod shaders;
pub mod states;
pub mod ui;
pub mod utils;

pub mod prelude {

    pub use std::f32::consts::*;

    pub use bevy::{prelude::*, reflect::*, winit::WinitSettings};

    pub use anyhow::{anyhow, bail, ensure, Result};
    pub use rand::prelude::*;

    #[cfg(feature = "debug")]
    pub use crate::debug::*;
    pub use crate::game::*;
    pub use crate::loading::*;
    pub use crate::player::*;
    pub use crate::polar::*;
    pub use crate::shaders::{cloud_cover::*, consts::*, earthlike::*, *};
    pub use crate::states::*;
    pub use crate::ui::*;
    pub use crate::utils::*;
}
