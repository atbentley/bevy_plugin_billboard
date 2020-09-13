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
    vec2 scale = vec2(1.0 / aspect, 1.0);
    vec3 particleCenter_wordspace = Model[3].xyz;
    vec3 vertexPosition_worldspace = particleCenter_wordspace;
    gl_Position = ViewProj * vec4(vertexPosition_worldspace, 1.0f);
    gl_Position /= gl_Position.w;
    gl_Position.xy += Vertex_Position.xy * scale * vec2(0.05, 0.05);

    v_Uv = Vertex_Uv;
    v_Normal = Vertex_Normal;
    v_Position = particleCenter_wordspace + Vertex_Position;
}
