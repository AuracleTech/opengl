#version 460 core
layout (location = 0) in vec3 a_pos;
layout (location = 1) in vec3 a_normal;
layout (location = 2) in vec2 a_texcoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec4 color;

out vec4 final_color;
out vec3 normal;
out vec2 tex_coord;

void main()
{
    gl_Position = projection * view * model * vec4(a_pos, 1.0);
    final_color = color;
    normal = a_normal;
    tex_coord = a_texcoord;
} 