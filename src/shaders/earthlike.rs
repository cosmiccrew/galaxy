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

add_celestial_shader_impl!(Earthlike);

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

#[cfg(test)]
mod test {

    use bevy::{asset::AssetPath, render::render_resource::ShaderRef, sprite::Material2d};

    use crate::prelude::*;

    #[test]
    fn test_material2d_impl() {
        let shader_ref = Earthlike::fragment_shader();

        let ShaderRef::Path(asset_path) = shader_ref else {

            panic!("\"ShaderRef\" from \"Earthlike::fragment_shader()\" isn't of enum variant \"ShaderRef::Path\"");

        };

        assert_eq!(
            asset_path,
            AssetPath::from("shaders/celestials/earthlike.wgsl")
        );
    }

    #[test]
    fn test_randomise() {
        let first = CloudCover::default();

        let mut second = first;

        //they should be equal here
        assert_eq!(first, second);

        second.randomise_seed();

        assert_ne!(first.celestial.seed, second.celestial.seed);

        second.randomise_rotation();

        assert_ne!(first.celestial.rotation, second.celestial.rotation);

        //reset
        second = first;

        second.randomise();

        assert_ne!(first, second);
    }

    #[test]
    fn test_celestial_impls() {
        let mut subject = Earthlike::default();

        let value = 123.456;

        subject.set_seed(value);
        subject.set_rotation(value);
        subject.set_pixels(value);
        subject.set_time_speed(value);

        assert_eq!(subject.celestial.seed, value);
        assert_eq!(subject.celestial.rotation, value);
        assert_eq!(subject.celestial.pixels, value);
        assert_eq!(subject.celestial.time_speed, value);
    }
}
