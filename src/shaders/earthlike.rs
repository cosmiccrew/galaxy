// use std::fmt::Debug;

use crate::prelude::*;
use bevy::{
    reflect::*,
    render::{extract_component::ExtractComponent, render_resource::*},
    sprite::Material2d,
};

#[derive(Debug, Component, Reflect, Clone, Copy, ShaderType, AsBindGroup, TypeUuid, PartialEq)]
#[reflect(Component)]
#[uuid = "aed9b1b9-229e-402a-b5a0-14d219af5d6d"]
pub struct Earthlike {
    #[uniform(0)]
    pub celestial: CelestialSettings,
    #[uniform(1)]
    pub land_colours: [Color; 4],
    #[uniform(2)]
    pub river_colours: [Color; 2],
}

impl Earthlike {
    pub(crate) fn randomise(&mut self) {
        self.randomise_rotation();
        self.randomise_seed();
    }

    pub(crate) fn randomise_seed(&mut self) {
        self.celestial.seed = rand::thread_rng().gen();
    }

    pub(crate) fn randomise_rotation(&mut self) {
        self.celestial.rotation = rand::thread_rng().gen_range(0f32..TAU);
    }
}

impl Default for Earthlike {
    fn default() -> Self {
        Self {
            celestial: Default::default(),
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

impl Material2d for Earthlike {
    fn fragment_shader() -> ShaderRef {
        "shaders/celestials/earthlike.wgsl".into()
    }
}

impl PlanetShader for Earthlike {}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_earthlike_randomise() {
        let first = Earthlike::default();

        let mut second = first;

        //they should be equal here
        assert_eq!(first, second);

        second.randomise();

        //now that second is randomised, it shouldn't have the same values as first!
        assert_ne!(first, second);
    }

    #[test]
    fn test_earthlike_randomise_seed() {
        let first = Earthlike::default();

        let mut second = first;

        //they should be equal here
        assert_eq!(first, second);

        second.randomise_seed();

        //now that second is randomised, it shouldn't have the same values as first!
        assert_ne!(first.celestial.seed, second.celestial.seed);
    }

    #[test]
    fn test_earthlike_randomise_rotation() {
        let first = Earthlike::default();

        let mut second = first;

        //they should be equal here
        assert_eq!(first, second);

        second.randomise_rotation();

        //now that second is randomised, it shouldn't have the same values as first!
        assert_ne!(first.celestial.rotation, second.celestial.rotation);
    }
}
