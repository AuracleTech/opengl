#version 330 core
in vec3 normal;
in vec3 frag_pos;

out vec4 frag_color;

uniform vec3 object_color;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 camera_pos;

void main()
{
    // diffuse
    vec3 norm = normalize(normal);
    vec3 light_dir = normalize(light_pos - frag_pos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_color;

    // ambient
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    // specular
    float specular_strength = 0.5;
    vec3 view_dir = normalize(camera_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light_color;

    vec3 result = (ambient + diffuse + specular) * object_color;
    frag_color = vec4(result, 1.0);
}