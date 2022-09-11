//
// Fragment Shader
//

#version 450 core

in VertexShaderOutput {
    vec4 Color;
    vec2 TexCoord;
} vertex;

out vec4 fragColor;

uniform vec2 iResolution;
uniform float iTime;
uniform float iTimeDelta;
uniform int iFrame;
uniform sampler2D iTexturex;

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec2 uv = (fragCoord-.5*iResolution.xy)/iResolution.y;
    float d = length(uv);
    float m = abs(sin(iTime))*0.2/d;
    vec3 col = vec3(m);
    fragColor = vec4(col, 1.0);
}

void main()
{
    vec2 fragCoord = gl_FragCoord.xy;
    mainImage(fragColor, fragCoord);
    fragColor += vertex.Color;
    fragColor += texture(iTexturex, vertex.TexCoord);
}
