#version 460 core
out vec4 frag_color;

struct Material {
    sampler2D albedo;
};
uniform Material material; // TEMP

void main()
{
    vec4 result = vec4(0.04, 0.28, 0.26, 1.0);
    result *= texture(material.albedo, vec2(0.0));
    frag_color = result;
}