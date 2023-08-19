use bevy::render::render_resource::ShaderType;

use crate::prelude::*;

#[derive(Default, Clone, ShaderType)]
pub struct GpuPlanetBuffer {
    count: f32,
    #[size(runtime)]
    pub data: Vec<GpuPlanet>,
}

/// A GPU ready implimentation of `Planet`.
#[derive(Default, Clone, ShaderType)]
pub struct GpuPlanet {
    pub seed: f32,
    pub pixels: f32,
    pub rotation: f32,
    pub radius: f32,
    pub time_speed: f32,
}
