use bevy::ecs::Bundle;
use bevy::prelude::*;
use bevy::render::{
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline},
    render_graph::base::MainPass,
};

use crate::mesh::BILLBOARD_MESH_HANDLE;
use crate::BILLBOARD_PIPELINE_HANDLE;

#[derive(Bundle)]
pub struct BillboardComponents {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for BillboardComponents {
    fn default() -> Self {
        BillboardComponents {
            mesh: BILLBOARD_MESH_HANDLE,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                BILLBOARD_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        DynamicBinding {
                            bind_group: 1,
                            binding: 0,
                        },
                        DynamicBinding {
                            bind_group: 1,
                            binding: 1,
                        },
                        DynamicBinding {
                            bind_group: 1,
                            binding: 2,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            material: Default::default(),
            draw: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}
