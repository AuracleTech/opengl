#version 460 core
out vec4 frag_color;
in vec4 final_color;
in vec3 normal;
in vec2 tex_coord;

void main()
{
    frag_color = final_color;
}