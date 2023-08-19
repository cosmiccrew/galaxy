use crate::prelude::*;

use bevy::{
    asset::Asset,
    core_pipeline::core_2d,
    ecs::component::TableStorage,
    prelude::shape::Plane,
    reflect::{TypePath, TypeUuid},
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::RenderGraph,
        render_resource::*,
        Extract, RenderApp,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

pub mod clouds;
pub mod earthlike;
pub mod types;

use self::types::{GpuCloudCover, GpuCloudCoverBuffer, GpuEarthlike, GpuEarthlikeBuffer};
pub use self::{clouds::*, earthlike::*};

/// Global settings used for every planet, regardless of its type or parameters.
#[derive(Resource, Reflect, Copy, Clone, InspectorOptions, ExtractResource)]
#[reflect(InspectorOptions)]
pub struct GlobalPlanetSettings {
    pub enabled: bool,
}

impl Default for GlobalPlanetSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

pub struct GalaxyShaderPlugin;

impl Plugin for GalaxyShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<GlobalPlanetSettings>::default(),
            // ExtractComponentPlugin::<Planet>::default(),
            // ExtractComponentPlugin::<Earthlike>::default(),
        ))
        .init_resource::<GlobalPlanetSettings>();

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(ExtractSchedule, system_extract_pipeline_assets);

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        // render_graph
        //     .add_node(PlanetPass2DNode::NAME, PlanetPass2DNode::default())
        //     .add_node_edges(
        //         // Specify the node ordering.
        //         // This will automatically create all required node edges to enforce the given ordering.
        //         &[
        //             core_2d::graph::node::MAIN_PASS,
        //             PlanetPass2DNode::NAME,
        //             core_2d::graph::node::TONEMAPPING,
        //         ],
        //     );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<PlanetShaderPipelineAssets>();
    }
}

// The post process node used for the render graph
#[derive(Default)]
struct PlanetPass2DNode;
impl PlanetPass2DNode {
    #[allow(dead_code)]
    pub const NAME: &str = "planet_pass_2d";
}

#[derive(Resource, Default)]
pub struct PlanetShaderPipelineAssets {
    pub earthlikes: StorageBuffer<GpuEarthlikeBuffer>,
    pub clouds: StorageBuffer<GpuCloudCoverBuffer>,
}

pub fn system_extract_pipeline_assets(
    res_planet_settings: Extract<Res<GlobalPlanetSettings>>,

    query_earthlike: Extract<Query<(&Transform, &Planet, &Earthlike, &ComputedVisibility)>>,

    query_clouds: Extract<Query<(&Transform, &Planet, &CloudCover, &ComputedVisibility)>>,

    query_planets_without_types: Extract<Query<(&Transform, &Planet, &ComputedVisibility)>>,

    mut gpu_pipeline_assets: ResMut<PlanetShaderPipelineAssets>,
) {
    let planet_settings = &res_planet_settings.enabled;

    {
        let earthlikes = gpu_pipeline_assets.earthlikes.get_mut();
        // let mut rng = thread_rng();
        earthlikes.count = 0;
        earthlikes.data.clear();
        for (transform, planet, earthlike, visibility) in query_earthlike.iter() {
            if visibility.is_visible() {
                earthlikes.count += 1;
                earthlikes.data.push(GpuEarthlike::new(*planet, *earthlike));
            }
        }
        println!("{:?}", earthlikes)
    }

    {
        let clouds = gpu_pipeline_assets.clouds.get_mut();
        // let mut rng = thread_rng();
        clouds.count = 0;
        clouds.data.clear();
        for (transform, planet, cloud, visibility) in query_clouds.iter() {
            if visibility.is_visible() {
                clouds.count += 1;
                clouds.data.push(GpuCloudCover::new(*planet, *cloud));
            }
        }
        println!("{:?}", clouds)
    }
}

/// Settings that each planet has, no matter what unique type the planet is (e.g. galaxies, earthlikes and fireworlds all have these), but that are individual (two differing )
#[derive(Component, Reflect, Debug, Clone, Copy, ShaderType)]
#[reflect(Component)]
pub struct Planet {
    /// The random seed that decides how this planet should be generated - this is used to generate a near inifinite amount of differing planets easily.
    pub seed: f32,
    /// how many pixels across the planet should be
    pub pixels: u32,
    /// a rotation in radians - therefore should be within the range: 0 -> TAU (TAU is 2 PI).
    ///
    /// This is needed rather than the rotation within `Transform, so that a planet can have its pixels aligned while being still rotated.
    pub rotation: f32,
    /// The radius occupied by the actual planet, seperate from its pixels - a planet can be 10 pixels wide but 1000 pixels of actual screen size, and likewise have 1000 pixels but only 100 of screen size.
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

pub trait PlanetShader: ShaderType + Component {}

// #[allow(clippy::large_enum_variant)]
// #[derive(Component, Reflect)]
// pub enum PlanetType {
//     Earthlike(Earthlike),
//     Moon(Earthlike),
//     // #[default]
//     // NoPlanet,
// }

// impl Default for PlanetType {
//     fn default() -> Self {
//         Self::Earthlike(Earthlike::default())
//     }
// }

#[derive(Bundle, Reflect, Default)]
pub struct PlanetBundle<P: PlanetShader> {
    pub planet: Planet,
    pub planet_shader: P,
    // pub clouds: Option<Clouds>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}
