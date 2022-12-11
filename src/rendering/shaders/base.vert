#version 450
#extension GL_ARB_seperate_shader_objects : enable

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(push_constant) uniform PushConstants{
   /* vec3 position;
    float scale;*/
    mat4 transform;
    vec4 color;
} push_constants;

layout(location = 0) out vec4 vertex_color;

void main(){
    vertex_color = push_constants.color;//vec4(0.6, 0.2, 0.8, 1.0);
    gl_Position = push_constants.transform * vec4(position, 1.0);
}