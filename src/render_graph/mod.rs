mod pipeline;
mod window_aspect_ratio;

use bevy::prelude::*;
use bevy::render::{
    pipeline::PipelineDescriptor,
    render_graph::base::node::MAIN_PASS,
    render_graph::{AssetRenderResourcesNode, RenderGraph},
    shader::Shader,
};

use crate::entity::BillboardMaterial;
use pipeline::{build_billboard_pipeline, build_billboard_xray_pipeline, build_shader};
use window_aspect_ratio::WindowAspectRatioNode;

pub const BILLBOARD_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(143751792813704602492294006880981632492);

pub const BILLBOARD_XRAY_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(139279763424867148995332307868825485376);

const BILLBOARD_NODE: &str = "billboard_material";
const WINDOW_ASPECT_RATIO_NODE: &str = "window_aspect_ratio";

pub fn add_billboard_graph(graph: &mut RenderGraph, resources: &Resources) {
    let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();

    graph.add_system_node(
        BILLBOARD_NODE,
        AssetRenderResourcesNode::<BillboardMaterial>::new(true),
    );
    graph.add_node_edge(BILLBOARD_NODE, MAIN_PASS).unwrap();
    graph.add_node(WINDOW_ASPECT_RATIO_NODE, WindowAspectRatioNode::default());
    graph
        .add_node_edge(WINDOW_ASPECT_RATIO_NODE, MAIN_PASS)
        .unwrap();

    let shader = build_shader(&mut shaders);
    pipelines.set(
        BILLBOARD_PIPELINE_HANDLE,
        build_billboard_pipeline(shader.clone()),
    );
    pipelines.set(
        BILLBOARD_XRAY_PIPELINE_HANDLE,
        build_billboard_xray_pipeline(shader),
    );
}
