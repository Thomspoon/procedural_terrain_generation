#version 330 core
out vec4 color;

uniform vec3 object_color;

void main()
{
    color = vec4(object_color, 1.0f);
}