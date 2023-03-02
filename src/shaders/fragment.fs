#version 330 core
struct Material {
    sampler2D diffuse_map;
    sampler2D specular_map;
    float specular_strength;
};
struct Light {
    vec3 pos;
    vec3 ambient_color;
    vec3 diffuse_color;
    vec3 specular_color;
};

in vec2 tex_coord;
in vec3 normal;
in vec3 frag_pos;

out vec4 frag_color;

uniform vec3 camera_pos;
uniform Material material;
uniform Light light;

void main()
{
    // ambient
    vec3 ambient = light.ambient_color * vec3(texture(material.diffuse_map, tex_coord));

    // diffuse
    vec3 norm = normalize(normal);
    vec3 light_dir = normalize(light.pos - frag_pos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = light.diffuse_color * diff * vec3(texture(material.diffuse_map, tex_coord));

    // specular
    vec3 view_dir = normalize(camera_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.specular_strength);
    vec3 specular = light.specular_color * spec * vec3(texture(material.specular_map, tex_coord));

    frag_color = vec4(ambient + diffuse + specular, 1.0);
}