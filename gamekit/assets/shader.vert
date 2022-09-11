//
// Vertex Shader
//

#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec2 TextureCoord;

out VertexShaderOutput {
    vec4 Color;
    vec2 TexCoord;
} vertex;

uniform vec2 iResolution;
uniform vec3 iMin;
uniform vec3 iMax;

void main()
{
    //float screen_width = iResolution.x > 0.0 ? iResolution.x : 1.0;
    //float screen_height = iResolution.y > 0.0 ? iResolution.y : 1.0;

    float width = iMax.x - iMin.y; if (width == 0.0) width = 1.0;
    float height = iMax.y - iMin.y; if (height == 0.0) height = 1.0;

    float x = -1.0 + 2.0 * (Position.x - iMin.x) / width;
    float y = 1.0 - 2.0 * (Position.y - iMin.y) / height;

    gl_Position = vec4(x, y, Position.z, 1.0);
    vertex.Color = Color;
    vertex.TexCoord = TextureCoord;
}
