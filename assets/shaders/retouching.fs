#version 460 core
in vec2 tex_coord;
  
out vec4 frag_color;

uniform sampler2D frame;

// TODO dither only 1 color channel, alternating between channels
// frag_color.rgb += dither(tex_coord);
float dither(vec2 uv)
{
    return fract(sin(dot(uv.xy, vec2(12.9898, 78.233))) * 43758.5453) / 256.0;
}

const float offset = 1.0 / 300.0;  

void main()
{
    frag_color = texture(frame, tex_coord);
    frag_color.rgb += dither(tex_coord);
}