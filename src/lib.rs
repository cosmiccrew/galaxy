// #![allow(unused)]
#![allow(clippy::type_complexity)]
// #![warn(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "debug")]
pub mod debug;
pub mod game;
pub mod loading;
pub mod player;
pub mod polar;
// pub mod shaders;
pub mod states;
pub mod ui;
pub mod utils;

pub mod cli;
pub mod physics;

pub mod prelude {

    pub use std::f32::consts::*;

    pub use bevy::{prelude::*, reflect::*, winit::WinitSettings};
    pub use bevy_xpbd_2d::prelude::*;
    pub use clap::Parser;
    pub use leafwing_input_manager::prelude::*;
    pub use log::{debug, error, info, trace, warn};
    pub use miette::{bail, ensure, miette, Result};
    pub use rand::prelude::*;

    #[cfg(feature = "debug")]
    pub use crate::debug::*;
    pub use crate::{
        cli::*,
        consts::*,
        game::*,
        loading::*,
        physics::*,
        player::*,
        polar::*,
        // shaders::{cloud_cover::*, consts::*, earthlike::*, *},
        states::*,
        ui::*,
        utils::*,
    };
}

pub mod consts {}
