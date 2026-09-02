#version 430 core

layout(location=0) out vec4 color;

//uniform float Time;          // Current elapsed time in seconds
//uniform float waitDur;



void main()
{
    color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    /*
    if(Time < waitDur){
        color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    }
    else {
        float activeTime = Time - waitDur;
        float red = 0.5 * (sin(activeTime) + 1.0);
        color = vec4(red, 0.5f, 1.0f, 1.0f);
    }
    */


}