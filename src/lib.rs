use bevy::prelude::*;
use bevy::render::{render_graph::RenderGraph, shader::asset_shader_defs_system};

mod entity;
mod mesh;
mod render_graph;

pub use entity::{BillboardComponents, BillboardMaterial};

use mesh::{BillboardMesh, BILLBOARD_MESH_HANDLE};
use render_graph::add_billboard_graph;

pub struct BillboardPlugin;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<BillboardMaterial>().add_system_to_stage(
            stage::POST_UPDATE,
            asset_shader_defs_system::<BillboardMaterial>.system(),
        );
        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        add_billboard_graph(&mut render_graph, resources);
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        meshes.set(BILLBOARD_MESH_HANDLE, BillboardMesh::default().into());
    }
}
