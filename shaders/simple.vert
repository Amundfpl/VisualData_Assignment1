#version 430 core

layout(location=2) in vec3 position;

mat3 mirrorH = mat3(
    1, 0, 0,
    0, -1, 0,
    0, 0, 1

);

mat3 mirrorV = mat3(
    -1, 0, 0,
    0, 1, 0,
    0, 0, 1

);

void main()
{
    // Task 2d: horizontal and vertical mirroring
    //vec3 transformedPosition = mirrorH * mirrorV * position;
    //gl_Position = vec4(transformedPosition, 1.0f);

    //default 
    gl_Position = vec4(position, 1.0f);
}