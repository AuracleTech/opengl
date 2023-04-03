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

void main()
{
    frag_color = texture(frame, tex_coord);
    float average = (frag_color.r + frag_color.g + frag_color.b) / 3.0;
    frag_color.rgb = vec3(average);

    float dither_value = dither(tex_coord);
    if (mod(gl_FragCoord.x, 2.0) < 1.0) {
        frag_color.rgb += dither_value;
    }
    if (mod(gl_FragCoord.y, 2.0) < 1.0) {
        frag_color.rgb -= dither_value;
    }
}