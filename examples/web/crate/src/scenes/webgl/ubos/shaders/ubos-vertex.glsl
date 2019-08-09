#version 300 es

precision mediump float;

in vec3 a_vertex;
in vec4 a_color;

out vec4 v_color;

layout (std140) uniform camera {
    uniform mat4 u_view;
    uniform mat4 u_projection;
};

layout (std140) uniform model {
    uniform mat4 u_size;
    uniform mat4 u_model;
};


layout (std140) uniform scale {
    uniform float u_scale_x;
    uniform float u_scale_y;
    uniform float u_scale_z;
};

void main() {
    mat4 scale_mat = mat4(1.0);
    scale_mat[0][0] = u_scale_x;
    scale_mat[1][1] = u_scale_y;
    scale_mat[2][2] = u_scale_z;
    mat4 size = u_size * scale_mat;

    //u_size[
    mat4 mvp = (u_projection * (u_view * u_model));
    gl_Position = mvp * (size * vec4(a_vertex,1));
    v_color = a_color;
}
