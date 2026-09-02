#version 430 core

layout(location=0) out vec4 color;

uniform float Time;          // task d.)

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
    //default 
    //color = vec4(1.0f, 1.0f, 1.0f, 1.0f);

    //optional task d.)
    changeColor();

    //optional task a.)
    //checkerBoard();
    


}