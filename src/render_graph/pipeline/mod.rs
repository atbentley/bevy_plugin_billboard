use bevy::prelude::*;
use bevy::render::{
    pipeline::{
        BlendDescriptor, BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite,
        CompareFunction, CullMode, DepthStencilStateDescriptor, FrontFace, PipelineDescriptor,
        RasterizationStateDescriptor, StencilStateDescriptor, StencilStateFaceDescriptor,
    },
    shader::{Shader, ShaderStage, ShaderStages},
    texture::TextureFormat,
};

pub fn build_shader(shaders: &mut Assets<Shader>) -> ShaderStages {
    let vertex = shaders.add(Shader::from_glsl(
        ShaderStage::Vertex,
        include_str!("./shader/shader.vert"),
    ));
    let fragment = Some(shaders.add(Shader::from_glsl(
        ShaderStage::Fragment,
        include_str!("./shader/shader.frag"),
    )));
    ShaderStages { vertex, fragment }
}

pub fn build_billboard_pipeline(shader: ShaderStages) -> PipelineDescriptor {
    build_billboard_pipeline_base(shader, CompareFunction::Less)
}

pub fn build_billboard_xray_pipeline(shader: ShaderStages) -> PipelineDescriptor {
    build_billboard_pipeline_base(shader, CompareFunction::Always)
}

fn build_billboard_pipeline_base(
    shader: ShaderStages,
    depth_compare: CompareFunction,
) -> PipelineDescriptor {
    PipelineDescriptor {
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        depth_stencil_state: Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }),
        color_states: vec![ColorStateDescriptor {
            format: TextureFormat::Bgra8UnormSrgb,
            color_blend: BlendDescriptor {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendDescriptor {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        ..PipelineDescriptor::new(shader)
    }
}
