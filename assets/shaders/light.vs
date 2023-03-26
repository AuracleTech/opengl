#version 460 core
layout (location = 0) in vec3 a_pos;
layout (location = 1) in vec3 a_normal;
layout (location = 2) in vec2 a_tex_coord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 normal;
out vec2 tex_coord;

void main()
{
    gl_Position = projection * view * model * vec4(a_pos, 1.0);
    normal = a_normal;
    tex_coord = a_tex_coord;
} 