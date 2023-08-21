use crate::prelude::*;
use bevy::{reflect::*, render::render_resource::*, sprite::Material2d};

#[derive(Component, Reflect, Debug, PartialEq, Clone, Copy, ShaderType, AsBindGroup, TypeUuid)]
#[reflect(Component)]
#[uuid = "2fa48e25-3736-4f6f-92fe-dfa60d0e1982"]
pub struct CloudCover {
    #[uniform(0)]
    pub celestial: CelestialSettings,
    #[uniform(1)]
    pub cloud_cover: f32,
    #[uniform(2)]
    pub colours: [Color; 4],
}

impl Default for CloudCover {
    fn default() -> Self {
        Self {
            celestial: Default::default(),
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

impl Material2d for CloudCover {
    fn fragment_shader() -> ShaderRef {
        "shaders/celestials/generic/cloud_cover.wgsl".into()
    }
}

add_celestial_shader_impl!(CloudCover);

#[cfg(test)]
mod test {

    use bevy::{asset::AssetPath, render::render_resource::ShaderRef, sprite::Material2d};

    use crate::prelude::*;

    #[test]
    fn test_material2d_impl() {
        let shader_ref = CloudCover::fragment_shader();

        let ShaderRef::Path(asset_path) = shader_ref else {

            panic!("\"ShaderRef\" from \"CloudCover::fragment_shader()\" isn't of enum variant \"ShaderRef::Path\"");

        };

        assert_eq!(
            asset_path,
            AssetPath::from("shaders/celestials/generic/cloud_cover.wgsl")
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
