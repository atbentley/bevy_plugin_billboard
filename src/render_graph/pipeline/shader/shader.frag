#version 450

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 3, binding = 0) uniform BillboardMaterial_albedo {
    vec4 Albedo;
};

# ifdef BILLBOARDMATERIAL_ALBEDO_TEXTURE
layout(set = 3, binding = 1) uniform texture2D BillboardMaterial_albedo_texture;
layout(set = 3, binding = 2) uniform sampler BillboardMaterial_albedo_texture_sampler;
# endif

void main() {
    vec4 output_color = Albedo;
# ifdef BILLBOARDMATERIAL_ALBEDO_TEXTURE
    output_color *= texture(
        sampler2D(BillboardMaterial_albedo_texture, BillboardMaterial_albedo_texture_sampler),
        v_Uv);
# endif

    o_Target = output_color;
}
