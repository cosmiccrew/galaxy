use bevy::render::render_resource::ShaderType;

use crate::prelude::*;

// #[derive(Default, Clone, ShaderType)]
// pub struct GpuPlanetBuffer {
//     count: u32,
//     #[size(runtime)]
//     pub data: Vec<Planet>,
// }

// /// A GPU ready implimentation of `Planet`.
// #[derive(Default, Clone, ShaderType)]
// pub struct GpuPlanet {
//     pub seed: f32,
//     pub pixels: f32,
//     pub rotation: f32,
//     pub radius: f32,
//     pub time_speed: f32,
// }

#[derive(Debug, Default, Clone, ShaderType)]
pub struct GpuEarthlikeBuffer {
    pub count: u32,
    #[size(runtime)]
    pub data: Vec<GpuEarthlike>,
}

#[derive(Debug, Default, Clone, ShaderType)]
pub struct GpuEarthlike {
    pub planet: Planet,
    pub earthlike: Earthlike,
}
impl GpuEarthlike {
    pub(crate) fn new(planet: Planet, earthlike: Earthlike) -> Self {
        Self { planet, earthlike }
    }
}

#[derive(Debug, Default, Clone, ShaderType)]
pub struct GpuCloudCoverBuffer {
    pub count: u32,
    #[size(runtime)]
    pub data: Vec<GpuCloudCover>,
}

#[derive(Debug, Default, Clone, ShaderType)]
pub struct GpuCloudCover {
    pub planet: Planet,
    pub cloud: CloudCover,
}
impl GpuCloudCover {
    pub(crate) fn new(planet: Planet, cloud: CloudCover) -> Self {
        Self { planet, cloud }
    }
}
