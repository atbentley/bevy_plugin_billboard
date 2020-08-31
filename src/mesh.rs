use bevy::prelude::*;
use bevy::render::mesh::VertexAttribute;
use bevy::render::pipeline::PrimitiveTopology;

pub const BILLBOARD_MESH_HANDLE: Handle<Mesh> =
    Handle::from_u128(123223657955276412638404101887414900100);

pub struct BillboardMesh {
    pub size: f32,
}

impl Default for BillboardMesh {
    fn default() -> Self {
        BillboardMesh { size: 1.0 }
    }
}

impl From<BillboardMesh> for Mesh {
    fn from(billboard_mesh: BillboardMesh) -> Self {
        let size = billboard_mesh.size;
        let vertices = &[
            ([-size, -size, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
            ([size, -size, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
            ([size, size, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
            ([-size, size, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
        ];

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(positions),
                VertexAttribute::normal(normals),
                VertexAttribute::uv(uvs),
            ],
            indices: Some(indices),
        }
    }
}
