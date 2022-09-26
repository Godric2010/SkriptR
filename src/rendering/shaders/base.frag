#version 450
#extension GL_ARB_seperate_shader_objects : enable

layout(location = 0) in vec4 vertex_color;
layout(location = 0) out vec4 fragment_color;

void main(){
    fragment_color = vertex_color;
}