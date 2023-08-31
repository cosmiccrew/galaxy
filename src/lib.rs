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

    pub use anyhow::{anyhow, bail, ensure, Result};
    pub use bevy::{prelude::*, reflect::*, winit::WinitSettings};
    pub use rand::prelude::*;

    #[cfg(feature = "debug")]
    pub use crate::debug::*;
    pub use crate::{
        consts::*,
        game::*,
        loading::*,
        player::*,
        polar::*,
        shaders::{cloud_cover::*, consts::*, earthlike::*, *},
        states::*,
        ui::*,
        utils::*,
    };
}

pub mod consts {

    pub const ASSETS_ROOT: &str = {
        #[cfg(feature = "bundle")]
        {
            #[cfg(target_os = "linux")]
            {
                "./assets"
            }
            #[cfg(target_os = "macos")]
            {
                "../Resources/assets"
            }
            #[cfg(target_os = "windows")]
            {
                "./assets"
            }
            #[cfg(target_family = "wasm")]
            {
                "./assets"
            }
        }

        #[cfg(not(all(
            feature = "bundle",
            any(
                target_os = "linux",
                target_os = "macos",
                target_os = "windows",
                target_family = "wasm"
            )
        )))]
        {
            "./assets"
        }
    };
}

// #[cfg(all(target_family = "wasm", feature = "debug"))]
// compile_error!("feature \"debug\" cannot be enabled for wasm targets!");
