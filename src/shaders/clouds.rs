use crate::prelude::*;
use bevy::{reflect::*, render::render_resource::*, sprite::Material2d};

#[derive(Component, Reflect)]
pub struct Clouds {
    pub cloud_cover: f32,
    pub colours: [Color; 4],
}

impl Default for Clouds {
    fn default() -> Self {
        Self {
            cloud_cover: 0.47,
            colours: [
                Color::rgb(0.960784, 1., 0.909804),
                Color::rgb(0.87451, 0.878431, 0.909804),
                Color::rgb(0.407843, 0.435294, 0.6),
                Color::rgb(0.25098, 0.286275, 0.45098),
            ],
        }
    }
}
