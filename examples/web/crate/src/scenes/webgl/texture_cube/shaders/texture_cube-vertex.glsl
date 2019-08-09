attribute vec3 a_vertex;

varying vec3 v_uv;

uniform mat4 u_modelViewProjection;
uniform mat4 u_size;

void main() {     
    gl_Position = u_modelViewProjection * (u_size * vec4(a_vertex,1));

    v_uv = a_vertex;
}
