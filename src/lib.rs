use bevy::prelude::*;
use bevy::render::{
    pipeline::PipelineDescriptor,
    render_graph::base::node::MAIN_PASS,
    render_graph::RenderGraph,
    shader::{Shader, ShaderStage, ShaderStages},
};

mod entity;
mod mesh;
mod window_aspect_ratio;

pub use entity::BillboardComponents;

use mesh::{BillboardMesh, BILLBOARD_MESH_HANDLE};
use window_aspect_ratio::{WindowAspectRatioNode, WINDOW_ASPECT_RATIO_NODE};

const BILLBOARD_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(143751792813704602492294006880981632492);

pub struct BillboardPlugin;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut render_graph: ResMut<RenderGraph>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    render_graph.add_node(WINDOW_ASPECT_RATIO_NODE, WindowAspectRatioNode::default());
    render_graph
        .add_node_edge(WINDOW_ASPECT_RATIO_NODE, MAIN_PASS)
        .unwrap();
    pipelines.set(
        BILLBOARD_PIPELINE_HANDLE,
        build_billboard_pipeline(&mut shaders),
    );
    meshes.set(BILLBOARD_MESH_HANDLE, BillboardMesh::default().into());
}

fn build_billboard_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    let vertex = shaders.add(Shader::from_glsl(
        ShaderStage::Vertex,
        include_str!("./shader/shader.vert"),
    ));
    let fragment = Some(shaders.add(Shader::from_glsl(
        ShaderStage::Fragment,
        include_str!("./shader/shader.frag"),
    )));
    let shader = ShaderStages { vertex, fragment };
    PipelineDescriptor::default_config(shader)
}
