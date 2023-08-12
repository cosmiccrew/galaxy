use crate::prelude::*;
use bevy::{reflect::*, render::render_resource::*, sprite::Material2d};

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "ad2446fb-f88e-47c9-9a85-c617603527c2"]
pub struct EarthlikeMaterial {
    /// a doc comment
    #[uniform(0)]
    pub pixels: f32,
    #[uniform(1)]
    pub seed: f32,
    #[uniform(2)]
    pub colours: [Color; 4],
}

impl Default for EarthlikeMaterial {
    fn default() -> Self {
        Self {
            pixels: 100.,
            seed: 8.98,
            colours: [
                [0.388235, 0.670588, 0.247059, 1.].into(),
                [0.231373, 0.490196, 0.309804, 1.].into(),
                [0.184314, 0.341176, 0.32549, 1.].into(),
                [0.156863, 0.207843, 0.25098, 1.].into(),
            ],
        }
    }
}

impl Material2d for EarthlikeMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/planets/earthlike.wgsl".into()
    }
}
