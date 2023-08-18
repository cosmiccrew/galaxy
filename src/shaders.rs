use crate::prelude::*;

use bevy::{
    asset::Asset,
    reflect::{TypePath, TypeUuid},
    render::{extract_resource::ExtractResourcePlugin, render_resource::*, RenderApp},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::InspectorOptions;

pub mod clouds;
pub mod earthlike;

pub use self::{clouds::*, earthlike::*};

#[derive(Resource, Default, Reflect, Copy, Clone, InspectorOptions)]
pub struct GlobalPlanetShaderSettings {
    pub enabled: bool,
}

pub struct GalaxyShaderPlugin;

impl Plugin for GalaxyShaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalPlanetShaderSettings>();

        let render_app = app.sub_app_mut(RenderApp);
    }
}

/// Settings that each planet has, no matter what unique type the planet is (e.g. galaxies, earthlikes and fireworlds all have these), but that are individual (two differing )
#[derive(Component, Reflect)]
pub struct Planet {
    /// The random seed that decides how this planet should be generated - this is used to generate a near inifinite amount of differing planets easily.
    pub seed: f32,
    /// how many pixels across the planet should be
    pub pixels: u16,
    /// a rotation in radians - therefore should be within the range: 0 -> TAU (TAU is 2 PI).
    ///
    /// This is needed rather than the rotation within `Transform, so that a planet can have its pixels aligned while being still rotated.
    pub rotation: f32,
    /// The raddius occupied by the actual planet, seperate from its pixels - a planet can be 10 pixels wide but 1000 pixels of actual screen size, and likewise have 1000 pixels but only 100 of screen size.
    pub radius: f32,
    /// How fast the planet rotated around its axis - this is equivalent to a seeting deciding whether it takes the earth 24hrs to do a full rotation or 2 minutes.
    ///
    /// a `time_speed` of 1. is equal to [UNKNOWN] seconds for a full rotation.
    pub time_speed: f32,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            seed: 8.98,
            pixels: 100,
            rotation: 0.,
            radius: 100.,
            time_speed: 0.2,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Component, Reflect)]
pub enum PlanetType {
    Earthlike(Earthlike),
    Moon,
    // #[default]
    // NoPlanet,
}

impl Default for PlanetType {
    fn default() -> Self {
        Self::Earthlike(Earthlike::default())
    }
}

#[derive(Component, Reflect, Default)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub planet_type: PlanetType,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}