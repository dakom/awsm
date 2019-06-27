precision mediump float;

uniform sampler2D u_sampler_smiley;
uniform sampler2D u_sampler_bridge;
varying vec2 v_uv;

void main() {
    vec4 color_1 = texture2D(u_sampler_smiley, v_uv); 
    vec4 color_2 = texture2D(u_sampler_bridge, v_uv); 
    gl_FragColor = color_1 * color_2;
}
