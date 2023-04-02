#version 460 core
out vec4 frag_color;
  
in vec2 tex_coord;

uniform sampler2D frame;

void main()
{ 
    frag_color = texture(frame, tex_coord);
}