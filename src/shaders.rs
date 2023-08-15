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
        // app.add_plugins((
        //     Material2dPlugin::<EarthlikeMaterial>::default(),
        //     Material2dPlugin::<CloudsMaterial>::default(),
        // ));

        // app.add_asset::<M>()
        //     .add_plugins(ExtractComponentPlugin::<Handle<M>>::extract_visible());
    }
}

#[derive(Component, Reflect)]
pub struct Planet {
    pub seed: f32,
    /// how many pixels across the planet should be
    pub pixels: u16,
    /// a rotation in radians - therefore should be within the range: 0 -> TAU (TAU is 2 PI).
    pub rotation: f32,
    ///
    pub radius: f32,
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

// #[derive(Bundle)]
// pub struct PlanetBundle {
//     // material_mesh_2d_bundle: MaterialMesh2dBundle<M>,
//     planet_settings: PlanetSettings,
//     _planet: Planet,
// }

// impl Default for PlanetBundle {
//     fn default() -> Self {
//         Self {
//             // material_mesh_2d_bundle: MaterialMesh2dBundle {
//             //     mesh: Default::default(),
//             //     material: Default::default(),
//             //     transform: Default::default(),
//             //     global_transform: Default::default(),
//             //     visibility: Default::default(),
//             //     computed_visibility: Default::default(),
//             // },
//             _planet: Planet,
//             planet_settings: PlanetSettings::default(),
//         }
//     }
// }

// #[derive(Component, Reflect)]
// pub struct Planet;

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

// #[derive(Component, Reflect, Default)]
// pub struct PlanetSettings {
//     pub planet_type: PlanetType,
//     pub pixels: u32,
//     pub speed: f32,
//     pub seed: i32,
//     pub mouse_pos: Vec2,
//     pub color: Color,
//     pub radius: f32,
// }

// impl PlanetSettings {}

// pub trait PlanetShader: Material2d {
//     ///
//     /// Using ideas from https://www.iquilezles.org/www/articles/palettes/palettes.htm
//     fn gen_new_colourscheme(n_colours: u32) -> Vec<Color> {
//         const SATURATION: f32 = 0.5;
//         const HUE_DIFF: f32 = 0.9;

//         let mut rng = rand::thread_rng();

//         let a = Vec3::new(
//             rng.gen_range(0f32..5f32),
//             rng.gen_range(0f32..5f32),
//             rng.gen_range(0f32..5f32),
//         );

//         let b = Vec3::splat(0.5) * SATURATION;
//         let c = Vec3::new(
//             rng.gen_range(0.5..1.5),
//             rng.gen_range(0.5..1.5),
//             rng.gen_range(0.5..1.5),
//         ) * HUE_DIFF;

//         let d = Vec3::new(
//             rng.gen_range(0f32..1f32),
//             rng.gen_range(0f32..1f32),
//             rng.gen_range(0f32..1f32),
//         ) * rng.gen_range(1f32..3f32);

//         let mut colours = vec![];

//         let n = ((n_colours as f32) - 1.).max(1.);

//         (0..n_colours).for_each(|i| {
//             let i = i as f32;
//             colours.push(Color::rgb(
//                 a.x + b.x * f32::cos(TAU * (c.x * (i / n) + d.x)),
//                 a.y + b.y * f32::cos(TAU * (c.y * (i / n) + d.y)),
//                 a.z + b.z * f32::cos(TAU * (c.z * (i / n) + d.z)),
//             ));
//         });

//         colours
//     }
// }
