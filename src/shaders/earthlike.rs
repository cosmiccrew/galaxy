// use std::fmt::Debug;

use crate::prelude::*;
use bevy::{reflect::*, render::render_resource::*, sprite::Material2d};

#[derive(Component, Reflect)]
pub struct Earthlike {
    pub land_colours: [Color; 4],
    pub river_colours: [Color; 2],
}

impl Default for Earthlike {
    fn default() -> Self {
        Self {
            land_colours: [
                Color::rgb(0.388235, 0.670588, 0.247059),
                Color::rgb(0.231373, 0.490196, 0.309804),
                Color::rgb(0.184314, 0.341176, 0.32549),
                Color::rgb(0.156863, 0.207843, 0.25098),
            ],
            river_colours: [
                Color::rgb(0.309804, 0.643137, 0.721569),
                Color::rgb(0.25098, 0.286275, 0.45098),
            ],
        }
    }
}

impl From<Earthlike> for PlanetType {
    fn from(value: Earthlike) -> Self {
        PlanetType::Earthlike(value)
    }
}
