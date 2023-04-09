#version 460 core
layout (location = 0) in vec3 a_pos;

out vec3 tex_coord;

uniform mat4 projection;
uniform mat4 view;

void main()
{
    tex_coord = a_pos;
    gl_Position = projection * view * vec4(a_pos, 1.0);
}