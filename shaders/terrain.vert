// Model view matrix vertex shader

#version 330 core
layout (location = 0) in vec3 a_pos;
layout (location = 1) in vec3 a_color;
layout (location = 2) in vec3 a_normal;

out vec3 fragment_pos;
out vec3 fragment_color;
out vec3 surface_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    fragment_pos = vec3(model * vec4(a_pos, 1.0));
    fragment_color = a_color;
    surface_normal = mat3(transpose(inverse(model))) * a_normal;

    gl_Position = projection * view * model * vec4(a_pos, 1.0f);
}