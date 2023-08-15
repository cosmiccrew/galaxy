use crate::prelude::*;
use bevy::{reflect::*, render::render_resource::*, sprite::Material2d};

// #[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
// #[uuid = "b8127607-981f-40d4-ab35-e98fe6bbf83b"]
// pub struct CloudsMaterial {
//     /// a doc comment
//     #[uniform(0)]
//     pub pixels: f32,
//     #[uniform(1)]
//     pub seed: f32,
//     #[uniform(2)]
//     pub colours: [Color; 4],
// }

// impl Default for CloudsMaterial {
//     fn default() -> Self {
//         Self {
//             pixels: 100.,
//             seed: 8.98,
//             colours: [
//                 [0.388235, 0.670588, 0.247059, 1.].into(),
//                 [0.231373, 0.490196, 0.309804, 1.].into(),
//                 [0.184314, 0.341176, 0.32549, 1.].into(),
//                 [0.156863, 0.207843, 0.25098, 1.].into(),
//             ],
//         }
//     }
// }

// impl Material2d for CloudsMaterial {
//     fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
//         "shaders/planets/earthlike.wgsl".into()
//     }
// }

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
