#version 300 es
precision mediump float;
precision highp sampler3D;

in vec2 v_uv;
out vec4 fragColor;

uniform sampler2D u_diffuse_sampler;
uniform sampler3D u_lut_sampler;
uniform float u_lut_size;
uniform bool u_lut_enabled;

void main () {
  //Thanks @mattdesl for the Three.JS reference here: https://github.com/mrdoob/three.js/pull/8124

  // Based on "GPU Gems 2 — Chapter 24. Using Lookup Tables to Accelerate Color Transformations"
  // More info and credits @ http://http.developer.nvidia.com/GPUGems2/gpugems2_chapter24.html
  vec4 rawColor = texture(u_diffuse_sampler, v_uv);

  // Compute the 3D LUT lookup scale/offset factor
  vec3 scale = vec3((u_lut_size - 1.0) / u_lut_size);
  vec3 offset = vec3(1.0 / (2.0 * u_lut_size));

  // Apply 3D LUT color transform
  if(u_lut_enabled) {
    fragColor.rgb = texture(u_lut_sampler, scale * rawColor.rgb + offset).rgb;
  } else {
    fragColor.rgb = rawColor.rgb;
  }

  fragColor.a = rawColor.a;

}
