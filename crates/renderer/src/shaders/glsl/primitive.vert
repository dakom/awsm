#version 300 es
precision mediump float;

layout (std140) uniform camera {
    uniform mat4 u_view;
    uniform mat4 u_projection;
};

uniform mat4 u_model;

in vec3 a_position;

void main() {

    //mat4 u_view_projection = mat4(1.2873, 0, 0, 0, 0, 2.4142, 0, 0, 0, 0, -1.0000, -1, 0, -0.6306, 0.8718, 0.8918);
    //mat4 u_view_projection = u_view * u_projection;
    //mat4 u_model = mat4(0.5224, 0, 0, 0, 0, 0.5224, 0, 0, 0, 0, 0.5224, 0, 0, 0, 0, 1);
    //mat4 u_mvp = u_view_projection * u_model;
    //gl_Position = u_mvp * vec4(a_position, 1.0); 

    mat4 view_projection = u_view * u_projection;
    mat4 model_view_projection = view_projection * u_model;
    gl_Position = model_view_projection * vec4(a_position, 1.0); 

    //TODO - upload...

    /*
    mat4 u_model = mat4(1.0);
    
    mat4 view = mat4(1.0); //u_view

    mat4 mv = (view * u_model);
    mat4 mvp = (u_projection * (view * u_model));

    vec4 old_position = gl_Position;
    gl_Position = mvp * vec4(a_position, 1.0); 
    */
}