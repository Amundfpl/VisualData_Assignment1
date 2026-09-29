#version 430 core

layout(location=0) out vec4 color;
layout(location=4) in vec4 rgba;
layout(location=6) in vec3 normals;

uniform float Time;          // task d.)

vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

void changeColor(){
        float red = 0.5 * (sin(Time) + 1.0);
        color = vec4(red, 0.5f, 1.0f, 1.0f);
}

void checkerBoard(){
    float x = gl_FragCoord.x;
    float y = gl_FragCoord.y;
    float box = 20.0;

    float stripes_x = floor(x/box);
    float stripes_y = floor(y/box);

    if(mod(stripes_x + stripes_y, 2.0) == 0.0){
        color = vec4(1.0f, 0.0f, 0.0f, 1.0f);
    }
    else{
        color = vec4(0.0f, 0.0f, 1.0f, 1.0f);
    }
}




void main()
{
    /*
    if (rgba.r > 0.5) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
    else {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
    */
    //default 
    //color = vec4(1.0f, 1.0f, 1.0f, 1.0f);

    //optional task d.)
    //changeColor();

    //optional task a.)
    //checkerBoard();
    //color = normals * max(0.0, normals * (- lightDirection));
    //color = vec4(normals, 1.0);
    vec3 colorRGB  = rgba.rgb;
    float lightIntensity = max(0.0, dot(normals, (- lightDirection) ));
    color = vec4(colorRGB * lightIntensity, 1.0);
    


}