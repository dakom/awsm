precision mediump float;

uniform samplerCube u_sampler;
varying vec3 v_uv;

void main() {
    gl_FragColor = textureCube(u_sampler, v_uv); 
}
