use crate::prelude::*;

use bevy::{
    asset::{load_internal_asset, Asset},
    core_pipeline::core_2d,
    ecs::component::TableStorage,
    prelude::shape::Plane,
    reflect::{TypePath, TypeUuid},
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::{self, RenderGraph},
        render_phase::AddRenderCommand,
        render_resource::*,
        renderer::{RenderContext, RenderDevice, RenderQueue},
        Extract, Render, RenderApp, RenderSet,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

pub mod cloud_cover;
pub mod consts;
pub mod earthlike;

pub use self::{cloud_cover::*, earthlike::*};

pub struct GalaxyShaderPlugin;

impl Plugin for GalaxyShaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_TYPES, "shaders/types.wgsl", Shader::from_wgsl);

        app.add_plugins((
            Material2dPlugin::<Earthlike>::default(),
            Material2dPlugin::<CloudCover>::default(),
        ));
    }
}

#[derive(Component, Reflect, Debug, Default, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Celestial;

/// Settings that each planet has, no matter what unique type the planet is (e.g. galaxies, earthlikes and fireworlds all have these), but that are individual (two differing )
#[derive(Component, Reflect, Debug, PartialEq, Clone, Copy, ShaderType)]
#[reflect(Component)]
pub struct CelestialSettings {
    /// The random seed that decides how this celestial should be generated - this is used to generate a near inifinite amount of differing celestials easily.
    pub seed: f32,
    /// how many pixels across the celestial should be
    ///
    /// despite this seeming logical to be of type u32 and not f32, for the sake of simplifying the shader this is an f32.
    pub pixels: f32,
    /// a rotation in radians - therefore should be within the range: 0 -> TAU (TAU is 2 PI).
    ///
    /// This is needed rather than the rotation within `Transform, so that a celestial can have its pixels aligned while being still rotated.
    pub rotation: f32,
    /// The radius occupied by the actual celestial, seperate from its pixels - a celestial can be 10 pixels wide but 1000 pixels of actual screen size, and likewise have 1000 pixels but only 100 of screen size.
    pub radius: f32,
    /// How fast the celestial rotated around its axis - this is equivalent to a seeting deciding whether it takes the earth 24hrs to do a full rotation or 2 minutes.
    ///
    /// a `time_speed` of 1. is equal to [UNKNOWN] seconds for a full rotation.
    pub time_speed: f32,
}

impl CelestialSettings {
    fn set_seed(&mut self, seed: f32) {
        self.seed = seed;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn set_pixels(&mut self, pixels: f32) {
        self.pixels = pixels;
    }

    fn set_time_speed(&mut self, time_speed: f32) {
        self.time_speed = time_speed;
    }
}

impl Default for CelestialSettings {
    fn default() -> Self {
        Self {
            seed: 8.98,
            pixels: 100.,
            rotation: 0.,
            radius: 100.,
            time_speed: 0.2,
        }
    }
}

pub trait CelestialShader: ShaderType + Component + AsBindGroup + Material2d {
    fn randomise(&mut self) {
        self.randomise_seed();
        self.randomise_rotation();
    }
    fn randomise_seed(&mut self) {
        self.set_seed(rand::thread_rng().gen());
    }
    fn randomise_rotation(&mut self) {
        self.set_rotation(rand::thread_rng().gen_range(0f32..TAU));
    }

    fn set_seed(&mut self, seed: f32);

    fn set_rotation(&mut self, rotation: f32);

    fn set_pixels(&mut self, pixels: f32);

    fn set_time_speed(&mut self, time_speed: f32);
}

#[macro_export]
macro_rules! add_celestial_shader_impl {
    ($struct_name:ident) => {
        impl CelestialShader for $struct_name {
            fn set_seed(&mut self, seed: f32) {
                self.celestial.set_seed(seed);
            }

            fn set_rotation(&mut self, rotation: f32) {
                self.celestial.set_rotation(rotation);
            }

            fn set_pixels(&mut self, pixels: f32) {
                self.celestial.set_pixels(pixels);
            }

            fn set_time_speed(&mut self, time_speed: f32) {
                self.celestial.set_time_speed(time_speed);
            }
        }
    };
}

#[derive(Bundle, Reflect, Default, Clone)]
pub struct CelestialBundle<P: CelestialShader> {
    pub celestial: Celestial,
    pub celestial_shader: Handle<P>,
    pub mesh: Mesh2dHandle,
    // pub clouds: Option<Clouds>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}
