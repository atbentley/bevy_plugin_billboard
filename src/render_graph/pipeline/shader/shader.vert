#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform WindowAspectRatio {
    float aspect;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    vec2 aspect_scale = vec2(1.0 / aspect, 1.0);
    vec2 scale = vec2(Model[0][0], Model[1][1]);
    gl_Position = ViewProj * vec4(Model[3].xyz, 1.0f);

# ifdef BILLBOARDMATERIAL_SCREEN_SPACE
    gl_Position /= gl_Position.w;
    gl_Position.xy += Vertex_Position.xy * aspect_scale * scale;
# else
    gl_Position.xy += Vertex_Position.xy * aspect_scale * scale;
# endif
    v_Uv = Vertex_Uv;
    v_Normal = Vertex_Normal;
    v_Position = gl_Position.xyz;
}
