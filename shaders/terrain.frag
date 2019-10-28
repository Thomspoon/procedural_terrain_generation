// Shader adapted from the following tutorials:
// https://learnopengl.com/Lighting/Basic-Lighting

#version 330 core
out vec4 color;

in vec3 fragment_pos;
in vec3 surface_normal;
in vec2 tex_coord;

uniform vec3 light_color;

uniform vec3 light_pos;
uniform sampler2D t_texture;

void main()
{
    // Ambient lighting
    float ambient_strength = 0.5;
    vec3 ambient = ambient_strength * light_color;

    // diffuse 
    vec3 norm = normalize(surface_normal);
    vec3 light_dir = normalize(light_pos - fragment_pos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_color;
        
    vec3 result = (ambient + diffuse) * texture(t_texture, tex_coord).xyz;
    color = vec4(result, 1.0f);
}