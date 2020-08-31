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
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform WindowAspectRatio {
    float aspect;
};

void main() {
    v_Normal = (Model * vec4(Vertex_Normal, 1.0)).xyz;
    v_Normal = mat3(Model) * Vertex_Normal;
    v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    v_Uv = Vertex_Uv;

    vec4 center = Model[3];
    vec2 scale = vec2(1.0 / aspect, 1.0);
    gl_Position = ViewProj
        * (Model * vec4(0.0, 0.0, 0.0, 1.0))
        + vec4(v_Position.xy * scale, v_Position.z, 1.0)
        - center;
}
