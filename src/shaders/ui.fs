#version 460 core
in vec2 tex_coord;
out vec4 frag_color;

uniform sampler2D tex;
uniform vec3 color;

void main()
{
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(tex, tex_coord).r);
    frag_color = vec4(color, 1.0) * sampled;
}