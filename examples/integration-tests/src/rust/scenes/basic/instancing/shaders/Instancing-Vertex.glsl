precision mediump float;

attribute vec2 a_vertex;
attribute vec2 a_position;

varying vec2 v_uv;

uniform mat4 u_size;
uniform mat4 u_camera;

void main() {
    mat4 modelViewProjection;

    //TODO - should be derived from u_camera a_position
    //What's here is an untested placeholder
    modelViewProjection = u_camera * a_position;

    gl_Position = modelViewProjection * (u_size * vec4(a_vertex,0,1));
    v_uv = a_vertex;
}
