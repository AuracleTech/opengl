#version 330 core
struct Material {
    sampler2D diffuse_map;
    sampler2D specular_map;
    float specular_strength;
};
struct Light {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};
struct DirLight {
    vec3 dir;
    Light light;
};
struct PointLight {
    vec3 pos;
    Light light;
};

in vec2 tex_coord;
in vec3 normal;
in vec3 frag_pos;

out vec4 frag_color;

uniform vec3 camera_pos;
uniform Material material;
uniform PointLight light;

void main()
{
    // ambient
    vec3 ambient = light.light.ambient * vec3(texture(material.diffuse_map, tex_coord));

    // diffuse
    vec3 norm = normalize(normal);
    vec3 light_dir = normalize(light.pos - frag_pos); // point light
    // vec3 light_dir = normalize(-light.dir); // directional light
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = light.light.diffuse * diff * vec3(texture(material.diffuse_map, tex_coord));

    // specular
    vec3 view_dir = normalize(camera_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.specular_strength);
    vec3 specular = light.light.specular * spec * vec3(texture(material.specular_map, tex_coord));

    // attenuation
    float distance = length(light.pos - frag_pos);
    float attenuation = 1.0  / (light.light.constant + light.light.linear * distance + light.light.quadratic * (distance * distance)); 

    ambient  *= attenuation; 
    diffuse  *= attenuation;
    specular *= attenuation;

    frag_color = vec4(ambient + diffuse + specular, 1.0);
}