use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
            BindGroupLayoutEntry, BindingType, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, MultisampleState, PipelineCache, PrimitiveState,
            RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor, ShaderStages,
            TextureFormat, TextureSampleType, TextureViewDimension,
        },
        renderer::RenderDevice,
        texture::BevyDefault,
    },
};

use crate::prelude::*;

// #[derive(Resource)]
// pub struct PlanetPassPipeline {
//     pub sdf_bind_group_layout: BindGroupLayout,
//     pub sdf_pipeline: CachedComputePipelineId,
//     pub ss_probe_bind_group_layout: BindGroupLayout,
//     pub ss_probe_pipeline: CachedComputePipelineId,
//     pub ss_bounce_bind_group_layout: BindGroupLayout,
//     pub ss_bounce_pipeline: CachedComputePipelineId,
//     pub ss_blend_bind_group_layout: BindGroupLayout,
//     pub ss_blend_pipeline: CachedComputePipelineId,
//     pub ss_filter_bind_group_layout: BindGroupLayout,
//     pub ss_filter_pipeline: CachedComputePipelineId,
// }

#[derive(Resource)]
pub struct PlanetPassPipeline {
    earthlike_bind_group_layout: BindGroupLayout,
    earthlike_pipeline_id: CachedRenderPipelineId,
    cloudcover_bind_group_layout: BindGroupLayout,
    cloudcover_pipeline_id: CachedRenderPipelineId,
}

pub(crate) fn system_queue_bind_groups(
    mut commands: Commands,
    pipeline: Res<PlanetPassPipeline>,
    gi_compute_assets: Res<PlanetPassPipelineAssets>,
    render_device: Res<RenderDevice>,
) {
    if let (Some(earthlikes), Some(clouds)) = (
        gi_compute_assets.earthlikes.binding(),
        gi_compute_assets.clouds.binding(),
    ) {
        let earthlike_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: "earthlike_bind_group".into(),
            layout: &pipeline.earthlike_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: earthlikes.clone(),
            }],
        });
    }

    // if let (
    //     Some(light_sources),
    //     Some(light_occluders),
    //     Some(camera_params),
    //     Some(gi_state),
    //     Some(probes),
    //     Some(skylight_masks),
    // ) = (
    //     gi_compute_assets.light_sources.binding(),
    //     gi_compute_assets.light_occluders.binding(),
    //     gi_compute_assets.camera_params.binding(),
    //     gi_compute_assets.light_pass_params.binding(),
    //     gi_compute_assets.probes.binding(),
    //     gi_compute_assets.skylight_masks.binding(),
    // ) {
    //     let targets = targets_wrapper
    //         .targets
    //         .as_ref()
    //         .expect("Targets should be initialized");

    //     let sdf_view_image = &gpu_images[&targets.sdf_target];
    //     let ss_probe_image = &gpu_images[&targets.ss_probe_target];
    //     let ss_bounce_image = &gpu_images[&targets.ss_bounce_target];
    //     let ss_blend_image = &gpu_images[&targets.ss_blend_target];
    //     let ss_filter_image = &gpu_images[&targets.ss_filter_target];
    //     let ss_pose_image = &gpu_images[&targets.ss_pose_target];

    //     let sdf_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    //         label: "gi_sdf_bind_group".into(),
    //         layout: &pipeline.sdf_bind_group_layout,
    //         entries: &[
    //             BindGroupEntry {
    //                 binding: 0,
    //                 resource: camera_params.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 1,
    //                 resource: light_occluders.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 2,
    //                 resource: BindingResource::TextureView(&sdf_view_image.texture_view),
    //             },
    //         ],
    //     });

    //     let ss_probe_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    //         label: "gi_ss_probe_bind_group".into(),
    //         layout: &pipeline.ss_probe_bind_group_layout,
    //         entries: &[
    //             BindGroupEntry {
    //                 binding: 0,
    //                 resource: camera_params.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 1,
    //                 resource: gi_state.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 2,
    //                 resource: probes.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 3,
    //                 resource: skylight_masks.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 4,
    //                 resource: light_sources.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 5,
    //                 resource: BindingResource::TextureView(&sdf_view_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 6,
    //                 resource: BindingResource::Sampler(&sdf_view_image.sampler),
    //             },
    //             BindGroupEntry {
    //                 binding: 7,
    //                 resource: BindingResource::TextureView(&ss_probe_image.texture_view),
    //             },
    //         ],
    //     });

    //     let ss_bounce_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    //         label: "gi_bounce_bind_group".into(),
    //         layout: &pipeline.ss_bounce_bind_group_layout,
    //         entries: &[
    //             BindGroupEntry {
    //                 binding: 0,
    //                 resource: camera_params.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 1,
    //                 resource: gi_state.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 2,
    //                 resource: BindingResource::TextureView(&sdf_view_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 3,
    //                 resource: BindingResource::Sampler(&sdf_view_image.sampler),
    //             },
    //             BindGroupEntry {
    //                 binding: 4,
    //                 resource: BindingResource::TextureView(&ss_probe_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 5,
    //                 resource: BindingResource::TextureView(&ss_bounce_image.texture_view),
    //             },
    //         ],
    //     });

    //     let ss_blend_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    //         label: "gi_blend_bind_group".into(),
    //         layout: &pipeline.ss_blend_bind_group_layout,
    //         entries: &[
    //             BindGroupEntry {
    //                 binding: 0,
    //                 resource: camera_params.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 1,
    //                 resource: gi_state.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 2,
    //                 resource: probes.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 3,
    //                 resource: BindingResource::TextureView(&sdf_view_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 4,
    //                 resource: BindingResource::Sampler(&sdf_view_image.sampler),
    //             },
    //             BindGroupEntry {
    //                 binding: 5,
    //                 resource: BindingResource::TextureView(&ss_bounce_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 6,
    //                 resource: BindingResource::TextureView(&ss_blend_image.texture_view),
    //             },
    //         ],
    //     });

    //     let ss_filter_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    //         label: "ss_filter_bind_group".into(),
    //         layout: &pipeline.ss_filter_bind_group_layout,
    //         entries: &[
    //             BindGroupEntry {
    //                 binding: 0,
    //                 resource: camera_params.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 1,
    //                 resource: gi_state.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 2,
    //                 resource: probes.clone(),
    //             },
    //             BindGroupEntry {
    //                 binding: 3,
    //                 resource: BindingResource::TextureView(&sdf_view_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 4,
    //                 resource: BindingResource::Sampler(&sdf_view_image.sampler),
    //             },
    //             BindGroupEntry {
    //                 binding: 5,
    //                 resource: BindingResource::TextureView(&ss_blend_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 6,
    //                 resource: BindingResource::TextureView(&ss_filter_image.texture_view),
    //             },
    //             BindGroupEntry {
    //                 binding: 7,
    //                 resource: BindingResource::TextureView(&ss_pose_image.texture_view),
    //             },
    //         ],
    //     });

    //     commands.insert_resource(LightPassPipelineBindGroups {
    //         sdf_bind_group,
    //         ss_probe_bind_group,
    //         ss_bounce_bind_group,
    //         ss_blend_bind_group,
    //         ss_filter_bind_group,
    //     });
    // }
}

impl FromWorld for PlanetPassPipeline {
    fn from_world(world: &mut World) -> Self {
        todo!()

        // let render_device = world.resource::<RenderDevice>();

        // // We need to define the bind group layout used for our pipeline
        // let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        //     label: Some("planet_pass_bind_group_layout"),
        //     entries: &[
        //         // The screen texture
        //         BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: ShaderStages::FRAGMENT,
        //             ty: BindingType::Texture {
        //                 sample_type: TextureSampleType::Float { filterable: true },
        //                 view_dimension: TextureViewDimension::D2,
        //                 multisampled: false,
        //             },
        //             count: None,
        //         },
        //         // The sampler that will be used to sample the screen texture
        //         BindGroupLayoutEntry {
        //             binding: 1,
        //             visibility: ShaderStages::FRAGMENT,
        //             ty: BindingType::Sampler(SamplerBindingType::Filtering),
        //             count: None,
        //         },
        //         // The settings uniform that will control the effect
        //         BindGroupLayoutEntry {
        //             binding: 2,
        //             visibility: ShaderStages::FRAGMENT,
        //             ty: BindingType::Buffer {
        //                 ty: bevy::render::render_resource::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: Some(PostProcessSettings::min_size()),
        //             },
        //             count: None,
        //         },
        //     ],
        // });

        // // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        // let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        // // Get the shader handle
        // let shader = world
        //     .resource::<AssetServer>()
        //     .load("shaders/planets/earthlike.wgsl");

        // let pipeline_id = world
        //     .resource_mut::<PipelineCache>()
        //     // This will add the pipeline to the cache and queue it's creation
        //     .queue_render_pipeline(RenderPipelineDescriptor {
        //         label: Some("planet_pass_pipeline".into()),
        //         layout: vec![layout.clone()],
        //         // This will setup a fullscreen triangle for the vertex state
        //         vertex: fullscreen_shader_vertex_state(),
        //         fragment: Some(FragmentState {
        //             shader,
        //             shader_defs: vec![],
        //             // Make sure this matches the entry point of your shader.
        //             // It can be anything as long as it matches here and in the shader.
        //             entry_point: "fragment".into(),
        //             targets: vec![Some(ColorTargetState {
        //                 format: TextureFormat::bevy_default(),
        //                 blend: None,
        //                 write_mask: ColorWrites::ALL,
        //             })],
        //         }),
        //         // All of the following property are not important for this effect so just use the default values.
        //         // This struct doesn't have the Default trai implemented because not all field can have a default value.
        //         primitive: PrimitiveState::default(),
        //         depth_stencil: None,
        //         multisample: MultisampleState::default(),
        //         push_constant_ranges: vec![],
        //     });

        // Self {
        //     layout,
        //     // sampler,
        //     pipeline_id,
        // }
    }
}
