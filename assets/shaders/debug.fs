#version 460 core
in vec2 tex_coord;
  
out vec4 frag_color;

uniform sampler2D frame;

// TODO dither only 1 color channel, alternating between channels
float dither(vec2 uv)
{
    return fract(sin(dot(uv.xy, vec2(12.9898, 78.233))) * 43758.5453) / 256.0;
}

void main()
{
    frag_color = vec4(vec3(1.0 - texture(frame, tex_coord).rgb), 1.0) + vec4(vec3(dither(tex_coord)), 1.0);
}