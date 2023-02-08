#version 330 core
out vec4 FragColor;  
in vec3 our_color;

void main()
{
    FragColor = vec4(our_color, 1.0);
}