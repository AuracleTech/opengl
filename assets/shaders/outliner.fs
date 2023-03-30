#version 460 core
out vec4 frag_color;

struct Material {
    sampler2D albedo;
};
uniform Material material; // TEMP

void main()
{
    vec4 result = vec4(0.58, 0.988, 0.012, 1.0);
    result += texture(material.albedo, vec2(0.0)) * 0.0001;
    frag_color = result;
}