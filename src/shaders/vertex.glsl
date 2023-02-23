#version 330 core
layout (location = 0) in vec3 a_pos;
layout (location = 1) in vec3 a_color;
layout (location = 2) in vec2 a_texcoord;

out vec3 our_color;
out vec2 tex_coord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    // note that we read the multiplication from right to left
    gl_Position = projection * view * model * vec4(a_pos, 1.0);
    our_color = a_color;
    tex_coord = a_texcoord;
}