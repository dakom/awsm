precision mediump float;

attribute vec3 a_vertex;
attribute vec4 a_color;

varying vec4 v_color;

uniform mat4 u_modelViewProjection;
uniform mat4 u_size;

void main() {
    gl_Position = u_modelViewProjection * (u_size * vec4(a_vertex,1));
    v_color = a_color;
}
