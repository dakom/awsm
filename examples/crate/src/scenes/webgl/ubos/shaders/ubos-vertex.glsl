#version 300 es

precision mediump float;

in vec3 a_vertex;
in vec4 a_color;

out vec4 v_color;

layout (std140) uniform camera {
    uniform mat4 u_projection;
    uniform mat4 u_view;
};

layout (std140) uniform model {
    uniform mat4 u_size;
    uniform mat4 u_model;
};


void main() {
    mat4 mvp = (u_projection * (u_view * u_model));
    gl_Position = mvp * (u_size * vec4(a_vertex,1));
    v_color = a_color;
}
