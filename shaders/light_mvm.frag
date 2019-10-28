// Shader adapted from the following tutorials:
// https://learnopengl.com/Lighting/Basic-Lighting

#version 330 core
out vec4 color;

in vec3 fragment_pos;
in vec3 surface_normal;

uniform vec3 light_pos;
uniform vec3 view_pos;
uniform vec3 light_color;
uniform vec3 object_color;

void main()
{
    // Ambient lighting
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    // diffuse 
    vec3 norm = normalize(surface_normal);
    vec3 light_dir = normalize(light_pos - fragment_pos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_color;
    
    // specular
    float specular_strength = 0.5;
    vec3 view_dir = normalize(view_pos - fragment_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);  
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light_color;  
        
    vec3 result = (ambient + diffuse + specular) * object_color;
    color = vec4(result, 1.0f);
}