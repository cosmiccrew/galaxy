use crate::{prelude::*, shaders::pipeline::PlanetPassPipeline};

use bevy::{
    asset::Asset,
    core_pipeline::core_2d,
    ecs::component::TableStorage,
    prelude::shape::Plane,
    reflect::{TypePath, TypeUuid},
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice, RenderQueue},
        Extract, Render, RenderApp, RenderSet, render_phase::AddRenderCommand,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

pub mod clouds;
pub mod earthlike;
pub mod pipeline;
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

        render_app
            .add_systems(ExtractSchedule, system_extract_pipeline_assets)
            .add_systems(
                Render,
                (
                    system_prepare_pipeline_assets.in_set(RenderSet::Prepare),
                    // system_queue_bind_groups.in_set(RenderSet::Queue),
                ),
            );

        // let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
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
// #[derive(Default)]
// struct PlanetPass2DNode;
// impl PlanetPass2DNode {
//     pub const NAME: &str = "planet_pass_2d";
// }

// impl render_graph::Node for PlanetPass2DNode {
//     fn update(&mut self, _world: &mut World) {}

//     #[rustfmt::skip]
//     fn run(
//         &self,
//         _: &mut render_graph::RenderGraphContext,
//         render_context: &mut RenderContext,
//         world: &World,
//     ) -> Result<(), render_graph::NodeRunError> {
//         // if let Some(pipeline_bind_groups) = world.get_resource::<PlanetPassPipelineBindGroups>() {
//             let pipeline_cache = world.resource::<PipelineCache>();
//             let pipeline = world.resource::<PlanetPassPipeline>();

//         //     if let (
//         //         Some(sdf_pipeline),
//         //         Some(ss_probe_pipeline),
//         //         Some(ss_bounce_pipeline),
//         //         Some(ss_blend_pipeline),
//         //         Some(ss_filter_pipeline),
//         //     ) = (
//         //         pipeline_cache.get_compute_pipeline(pipeline.sdf_pipeline),
//         //         pipeline_cache.get_compute_pipeline(pipeline.ss_probe_pipeline),
//         //         pipeline_cache.get_compute_pipeline(pipeline.ss_bounce_pipeline),
//         //         pipeline_cache.get_compute_pipeline(pipeline.ss_blend_pipeline),
//         //         pipeline_cache.get_compute_pipeline(pipeline.ss_filter_pipeline),
//         //     ) {
//         //         let primary_w = target_sizes.primary_target_usize.x;
//         //         let primary_h = target_sizes.primary_target_usize.y;
//         //         let sdf_w = target_sizes.sdf_target_usize.x;
//         //         let sdf_h = target_sizes.sdf_target_usize.y;

//         //         let mut pass =
//         //             render_context
//         //                 .command_encoder()
//         //                 .begin_compute_pass(&ComputePassDescriptor {
//         //                     label: Some("light_pass_2d"),
//         //                 });

//         //         {
//         //             let grid_w = sdf_w / WORKGROUP_SIZE;
//         //             let grid_h = sdf_h / WORKGROUP_SIZE;
//         //             pass.set_bind_group(0, &pipeline_bind_groups.sdf_bind_group, &[]);
//         //             pass.set_pipeline(sdf_pipeline);
//         //             pass.dispatch_workgroups(grid_w, grid_h, 1);
//         //         }

//         //         {
//         //             let grid_w = (primary_w / GI_SCREEN_PROBE_SIZE as u32) / WORKGROUP_SIZE;
//         //             let grid_h = (primary_h / GI_SCREEN_PROBE_SIZE as u32) / WORKGROUP_SIZE;
//         //             pass.set_bind_group(0, &pipeline_bind_groups.ss_probe_bind_group, &[]);
//         //             pass.set_pipeline(ss_probe_pipeline);
//         //             pass.dispatch_workgroups(grid_w, grid_h, 1);
//         //         }

//         //         {
//         //             let grid_w = (primary_w / GI_SCREEN_PROBE_SIZE as u32) / WORKGROUP_SIZE;
//         //             let grid_h = (primary_h / GI_SCREEN_PROBE_SIZE as u32) / WORKGROUP_SIZE;
//         //             pass.set_bind_group(0, &pipeline_bind_groups.ss_bounce_bind_group, &[]);
//         //             pass.set_pipeline(ss_bounce_pipeline);
//         //             pass.dispatch_workgroups(grid_w, grid_h, 1);
//         //         }

//         //         {
//         //             let grid_w = (primary_w / GI_SCREEN_PROBE_SIZE as u32) / WORKGROUP_SIZE;
//         //             let grid_h = (primary_h / GI_SCREEN_PROBE_SIZE as u32) / WORKGROUP_SIZE;
//         //             pass.set_bind_group(0, &pipeline_bind_groups.ss_blend_bind_group, &[]);
//         //             pass.set_pipeline(ss_blend_pipeline);
//         //             pass.dispatch_workgroups(grid_w, grid_h, 1);
//         //         }

//         //         {
//         //             let grid_w = primary_w / WORKGROUP_SIZE;
//         //             let grid_h = primary_h / WORKGROUP_SIZE;
//         //             pass.set_bind_group(0, &pipeline_bind_groups.ss_filter_bind_group, &[]);
//         //             pass.set_pipeline(ss_filter_pipeline);
//         //             pass.dispatch_workgroups(grid_w, grid_h, 1);
//         //         }
//         //     }
//         // } else {
//         //     log::warn!("Failed to get bind groups");
//         // }

//         // Ok(())
//         todo!()
//     }
// }

#[derive(Resource, Default)]
pub struct PlanetShaderPipelineAssets {
    pub earthlikes: StorageBuffer<GpuEarthlikeBuffer>,
    pub clouds: StorageBuffer<GpuCloudCoverBuffer>,
}

impl PlanetShaderPipelineAssets {
    pub(crate) fn write_buffer(&mut self, device: &RenderDevice, queue: &RenderQueue) {
        self.earthlikes.write_buffer(device, queue);
        self.clouds.write_buffer(device, queue);
        // self.camera_params.write_buffer(device, queue);
        // self.light_pass_params.write_buffer(device, queue);
        // self.probes.write_buffer(device, queue);
        // self.skylight_masks.write_buffer(device, queue);
    }
}

pub(crate) fn system_prepare_pipeline_assets(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut gi_compute_assets: ResMut<PlanetShaderPipelineAssets>,
) {
    gi_compute_assets.write_buffer(&render_device, &render_queue);
}

pub(crate) fn system_extract_pipeline_assets(
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
