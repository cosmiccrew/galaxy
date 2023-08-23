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

impl CloudCover {
    pub(crate) fn randomise_seed(&mut self) {
        self.celestial.seed = (rand::thread_rng().gen());
    }
    pub(crate) fn randomise_rotation(&mut self) {
        self.celestial.rotation = (rand::thread_rng().gen_range(0f32..TAU));
    }
}

impl CelestialShader for CloudCover {
    fn randomise(&mut self) {
        self.randomise_seed();
        self.randomise_rotation();
    }
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

#[cfg(test)]
mod test {

    use crate::prelude::*;

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
}
