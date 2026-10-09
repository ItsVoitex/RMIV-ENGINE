#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
out vec2 texCoord;
uniform vec3 offset;
uniform vec3 scale;
void main()
{
    vec3 position = aPos * scale;
    position = position + offset;
    position.x = position.x  / 960;
    position.y = position.y / 540 ;
    gl_Position = vec4(position, 1.0);
    texCoord = aTexCoord;
}