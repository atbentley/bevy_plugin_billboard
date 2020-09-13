use bevy::{
    ecs::Bundle,
    prelude::*,
    render::{
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::base,
        renderer::RenderResources,
        shader::ShaderDefs,
    },
};

use crate::mesh::BILLBOARD_MESH_HANDLE;
use crate::render_graph::{BILLBOARD_PIPELINE_HANDLE, BILLBOARD_XRAY_PIPELINE_HANDLE};

#[derive(RenderResources, ShaderDefs)]
pub struct BillboardMaterial {
    pub albedo: Color,
    #[shader_def]
    pub albedo_texture: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub screen_space: bool,
}

impl Default for BillboardMaterial {
    fn default() -> Self {
        BillboardMaterial {
            albedo: Color::rgb(1.0, 1.0, 1.0),
            albedo_texture: None,
            screen_space: false,
        }
    }
}

#[derive(Bundle)]
pub struct BillboardComponents {
    pub billboard: Handle<BillboardMaterial>,
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub main_pass: base::MainPass,
    pub transform: Transform,
    pub translation: Translation,
    pub scale: Scale,
}

impl Default for BillboardComponents {
    fn default() -> Self {
        Self {
            render_pipelines: render_pipeline(BILLBOARD_PIPELINE_HANDLE),
            billboard: Default::default(),
            mesh: BILLBOARD_MESH_HANDLE,
            draw: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            scale: Default::default(),
        }
    }
}

impl BillboardComponents {
    pub fn xray() -> Self {
        Self {
            render_pipelines: render_pipeline(BILLBOARD_XRAY_PIPELINE_HANDLE),
            ..Default::default()
        }
    }
}

fn render_pipeline(handle: Handle<PipelineDescriptor>) -> RenderPipelines {
    RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
        handle,
        PipelineSpecialization {
            dynamic_bindings: vec![
                // transform
                DynamicBinding {
                    bind_group: 2,
                    binding: 0,
                },
                // billboard
                DynamicBinding {
                    bind_group: 3,
                    binding: 0,
                },
            ],
            ..Default::default()
        },
    )])
}
