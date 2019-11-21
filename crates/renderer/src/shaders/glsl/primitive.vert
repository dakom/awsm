#version 300 es
precision mediump float;

layout (std140) uniform camera {
    uniform mat4 u_view;
    uniform mat4 u_projection;
};

in vec3 a_position;

void main() {

    //TODO - upload...
    mat4 u_model = mat4(1.0);
    
    mat4 view = mat4(1.0); //u_view

    mat4 mv = (view * u_model);
    mat4 mvp = (u_projection * (view * u_model));

    gl_Position = mvp * vec4(a_position, 1.0); 
}