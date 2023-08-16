use std::fmt::Debug;

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
    /// Should only be a value between 0 and TAU.
    #[uniform(2)]
    pub rotation: f32,
    #[uniform(3)]
    pub colours: [Color; 4],
}

impl Default for EarthlikeMaterial {
    fn default() -> Self {
        Self {
            pixels: 100.,
            seed: 8.98,
            rotation: 0.75,
            colours: [
                [0.388235, 0.670588, 0.247059, 1.].into(),
                [0.231373, 0.490196, 0.309804, 1.].into(),
                [0.184314, 0.341176, 0.32549, 1.].into(),
                [0.156863, 0.207843, 0.25098, 1.].into(),
            ],
        }
    }
}

impl PlanetShader for EarthlikeMaterial {}

impl EarthlikeMaterial {
    pub fn randomise(&mut self) {
        self.randomise_rotation();
        self.randomise_seed();
        // self.randomise_colours(n_colours);
    }

    fn randomise_rotation(&mut self) {
        self.rotation = rand::thread_rng().gen_range(0f32..TAU);
    }

    fn randomise_seed(&mut self) {
        self.seed = rand::thread_rng().gen::<f32>();
    }

    // fn randomise_colours(&mut self, n_colours: u32) {
    //     let seed_colours = Self::gen_new_colourscheme(n_colours);

    //     // seed_colours.into_iter().for_each(|colour| {

    //     seed_colours.into_iter().for_each(|| {

    //         let difference = Vec4::from(colour1) - Vec4::from(colour2);

    //         self.colours.push(value);

    //     });
    // }
}

impl Material2d for EarthlikeMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/planets/earthlike.wgsl".into()
    }
}
