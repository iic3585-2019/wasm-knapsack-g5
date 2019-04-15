#include <stdio.h>
#include <emscripten/emscripten.h>

float EMSCRIPTEN_KEEPALIVE equation(float a, float b, float c)
{
     float Y;
     if(a == 0)
     {
          return 0;
     }
     else
     {
          Y = -(b + c) / a;
     }
    return Y;
}
