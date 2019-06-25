#version 300 es

precision mediump float;

in vec3 a_vertex;
in vec4 a_color;

out vec4 v_color;

layout (std140) uniform camera_and_size {
    uniform mat4 u_size;
    uniform mat4 u_modelViewProjection;
};

void main() {
    gl_Position = u_modelViewProjection * (u_size * vec4(a_vertex,1));
    v_color = a_color;
}
