#version 330 core

in vec2 tex_coord;

out vec4 frag_color;

uniform sampler2D texture_frame;
uniform sampler2D texture_flume;

void main()
{
    frag_color = mix(texture(texture_frame, tex_coord), texture(texture_flume, tex_coord), 0.2);
}