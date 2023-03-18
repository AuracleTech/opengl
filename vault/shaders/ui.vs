#version 460 core
layout (location = 0) in vec4 a_vertex;

out vec2 tex_coord;

uniform mat4 projection;

void main()
{
    gl_Position = projection * vec4(a_vertex.xy, 0.0, 1.0);
    tex_coord = a_vertex.zw;
}