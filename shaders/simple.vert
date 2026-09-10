#version 430 core

layout(location=2) in vec3 position;
layout(location=3) in vec4 rgba;
layout(location=4) out vec4 Vcolor;

uniform float Time;

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

float test = sin(Time);

float a = 1.0;
float b = 0.0;
float c = 0.0;
float d = 0.0;
float e = 1.0;
float f = 0.0;

mat4 transform = mat4(
    a, d, 0.0, 0.0,
    b, e, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    c, f, 0.0, 1.0
    );


void main()
{
    // Task 2d: horizontal and vertical mirroring
    //vec3 transformedPosition = mirrorH * mirrorV * position;
    //gl_Position = vec4(transformedPosition, 1.0f);

    //default 
    
    vec4 newPosition = transform * vec4(position, 1.0);
    gl_Position = newPosition;
    Vcolor = rgba;
}