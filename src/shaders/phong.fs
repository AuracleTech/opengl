#version 460 core
out vec4 frag_color;

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float specular_strength;
    sampler2D emissive;
}; 

struct DirLight {
    vec3 dir;
	
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 pos;
    
    float constant;
    float linear;
    float quadratic;
	
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct SpotLight {
    vec3 pos;
    vec3 dir;

    float cut_off;
    float outer_cut_off;
  
    float constant;
    float linear;
    float quadratic;
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;       
};

#define NR_POINT_LIGHTS 4

in vec2 tex_coord;
in vec3 normal;
in vec3 frag_pos;

uniform vec3 camera_pos;
uniform DirLight dirlight;
uniform PointLight pointlights[NR_POINT_LIGHTS];
uniform SpotLight spotlight;
uniform Material material;

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 view_dir);
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 frag_pos, vec3 view_dir);
vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 frag_pos, vec3 view_dir);

void main()
{
    vec3 norm = normalize(normal);
    vec3 view_dir = normalize(camera_pos - frag_pos);
    
    vec3 result = CalcDirLight(dirlight, norm, view_dir);
    for(int i = 0; i < NR_POINT_LIGHTS; i++)
        result += CalcPointLight(pointlights[i], norm, frag_pos, view_dir);
    result += CalcSpotLight(spotlight, norm, frag_pos, view_dir);

    vec3 emissive = texture(material.emissive, tex_coord).rgb;
    emissive *= 2.0; // increase the emissive effect
    result += emissive;
    
    frag_color = vec4(result, 1.0);
}

// calculates the color when using a directional light.
vec3 CalcDirLight(DirLight light, vec3 normal, vec3 view_dir)
{
    vec3 light_dir = normalize(-light.dir);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflectDir), 0.0), material.specular_strength);
    // combine results
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, tex_coord));
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, tex_coord));
    vec3 specular = light.specular * spec * vec3(texture(material.specular, tex_coord));
    return (ambient + diffuse + specular);
}

// calculates the color when using a point light.
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 frag_pos, vec3 view_dir)
{
    vec3 light_dir = normalize(light.pos - frag_pos);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflectDir), 0.0), material.specular_strength);
    // attenuation
    float distance = length(light.pos - frag_pos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));    
    // combine results
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, tex_coord));
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, tex_coord));
    vec3 specular = light.specular * spec * vec3(texture(material.specular, tex_coord));
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

// calculates the color when using a spot light.
vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 frag_pos, vec3 view_dir)
{
    vec3 light_dir = normalize(light.pos - frag_pos);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflectDir), 0.0), material.specular_strength);
    // attenuation
    float distance = length(light.pos - frag_pos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));    
    // spotlight intensity
    float theta = dot(light_dir, normalize(-light.dir)); 
    float epsilon = light.cut_off - light.outer_cut_off;
    float intensity = clamp((theta - light.outer_cut_off) / epsilon, 0.0, 1.0);
    // combine results
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, tex_coord));
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, tex_coord));
    vec3 specular = light.specular * spec * vec3(texture(material.specular, tex_coord));
    ambient *= attenuation * intensity;
    diffuse *= attenuation * intensity;
    specular *= attenuation * intensity;
    return (ambient + diffuse + specular);
}