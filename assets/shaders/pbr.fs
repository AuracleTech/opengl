#version 460 core
out vec4 frag_color;

in vec3 normal;
in vec2 tex_coord;
in vec3 frag_pos;

struct Material {
    sampler2D albedo;
};
uniform Material material;

// TODO move to ???
float near = 0.1; // TODO use camera near
float far  = 100.0;  // TODO use camera far
float LinearizeDepth(float depth) 
{
    float depth_normalized = depth * 2.0 - 1.0;
    return (2.0 * near * far) / (far + near - depth_normalized * (far - near));	
}

// TODO move dither in post process
float dither(vec2 uv)
{
    return fract(sin(dot(uv.xy, vec2(12.9898, 78.233))) * 43758.5453) / 256.0 - 0.001953125;
}

void main()
{
    // vec4 result = vec4(0.0);

    // MATERIAL ALBEDO
    // vec3 norm = normalize(normal);
    // vec4 albedo = texture(material.albedo, tex_coord);
    
    // DEPTH
    // float depth = LinearizeDepth(gl_FragCoord.z) / far; // divide by far for demonstration
    // result += vec4(vec3(depth), 1.0);

    // DITHER
    // result += vec4(vec3(dither(tex_coord)), 1.0);

    frag_color = texture(material.albedo, tex_coord);
}