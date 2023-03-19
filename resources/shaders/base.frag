#version 450
#extension GL_ARB_seperate_shader_objects : enable

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

layout(set = 1, binding = 0) uniform UBOCol{
    vec4 color;
} color_dat;

void main(){
    fragment_color = texture(sampler2D(u_texture, u_sampler), v_uv) * color_dat.color;
}