#version 450
#extension GL_ARB_seperate_shader_objects : enable

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 0) out vec2 v_uv;

layout(push_constant) uniform PushConstants{
   /* vec3 position;
    float scale;*/
    mat4 model;
    mat4 view;
    mat4 projection;
} push_constants;

//layout(location = 0) out vec4 vertex_color;

void main(){
    v_uv = uv;
    gl_Position = push_constants.projection * push_constants.view * push_constants.model * vec4(position, 1.0);
}