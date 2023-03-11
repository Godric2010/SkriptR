#version 450
#extension GL_ARB_seperate_shader_objects : enable

layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0) uniform UBOCol{
    vec4 color;
} color_dat;

void main(){
    fragment_color = color_dat.color;
}