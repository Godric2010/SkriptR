#version 450
#extension GL_ARB_seperate_shader_objects : enable

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(push_constant) uniform PushConstants{
    vec3 position;
    float scale;
} push_constants;

layout(location = 0) out vec4 vertex_color;

void main(){
    vertex_color = vec4(0.6, 0.2, 0.8, 1.0);
    vec3 scaled_position = vec3(position.rgb * push_constants.scale);
    gl_Position = /*vec4(push_constants.position, 0.0) + */vec4(scaled_position, 1.0);
}