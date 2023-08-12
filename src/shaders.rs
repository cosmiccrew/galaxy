use crate::prelude::*;

use bevy::{
    asset::Asset,
    reflect::{TypePath, TypeUuid},
    render::{extract_resource::ExtractResourcePlugin, render_resource::*},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

pub mod clouds;
pub mod earthlike;

pub use self::{clouds::*, earthlike::*};

pub struct GalaxyShaderPlugin;

impl Plugin for GalaxyShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Material2dPlugin::<EarthlikeMaterial>::default(),
            Material2dPlugin::<CloudsMaterial>::default(),
        ));
    }
}

#[derive(Bundle)]
pub struct PlanetBundle {
    // material_mesh_2d_bundle: MaterialMesh2dBundle<M>,
    planet_settings: PlanetSettings,
    _planet: Planet,
}

impl Default for PlanetBundle {
    fn default() -> Self {
        Self {
            // material_mesh_2d_bundle: MaterialMesh2dBundle {
            //     mesh: Default::default(),
            //     material: Default::default(),
            //     transform: Default::default(),
            //     global_transform: Default::default(),
            //     visibility: Default::default(),
            //     computed_visibility: Default::default(),
            // },
            _planet: Planet,
            planet_settings: PlanetSettings::default(),
        }
    }
}

#[derive(Component, Reflect)]
pub struct Planet;

#[derive(Component, Reflect, Default /* , ShaderSet*/)]
pub enum PlanetType {
    // #[shader = EarthlikeShader]
    #[default]
    Earthlike,
    NotRealYet,
}

#[derive(Component, Reflect, Default)]
pub struct PlanetSettings {
    pub planet_type: PlanetType,
    pub pixels: u32,
    pub speed: f32,
    pub seed: i32,
    pub mouse_pos: Vec2,
    pub color: Color,
    pub radius: f32,
}

impl PlanetSettings {}
