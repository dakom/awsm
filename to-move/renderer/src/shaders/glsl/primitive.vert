#version 300 es
precision mediump float;

layout (std140) uniform camera {
    uniform mat4 u_view;
    uniform mat4 u_projection;
};

uniform mat4 u_model;

in vec3 a_position;

void main() {
    mat4 vp = u_projection * u_view;
    mat4 mvp = vp * u_model;
    gl_Position = mvp * vec4(a_position, 1.0); 
}