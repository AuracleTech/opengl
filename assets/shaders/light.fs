#version 460 core
out vec4 frag_color;

in vec3 normal;

void main()
{
    frag_color = vec4(normal, 1.0) * 0.5 + 0.5;
} 