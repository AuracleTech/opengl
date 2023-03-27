#version 460 core
out vec4 frag_color;

in vec3 normal;
in vec2 tex_coord;
in vec3 frag_pos;

uniform sampler2D albedo;

void main()
{
    vec3 norm = normalize(normal);
    vec3 diffuse = texture(albedo, tex_coord).rgb;
    frag_color = vec4(diffuse, 1.0) + 0.1;
}