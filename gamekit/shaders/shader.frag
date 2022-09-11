//
// Fragment Shader
//

#version 450 core

layout(std140, binding = 1) buffer data_buffer {
    float resolution_x;
    float resolution_y;
    float x_min;
    float x_max;
    float y_min;
    float y_max;
    float time;
    float time_delta;
    int frame;
} data;

layout(std140, binding = 2) uniform uniform_buffer {
    float a;
    float b;
} uniforms;

layout (binding = 0) uniform sampler2D iTexture;

layout (location = 0) in vertex_data {
    vec4 position;
    vec4 color;
    vec2 textureCoord;
} inputs;

layout (location = 0) out vec4 oColor;

void calculate_color( out vec4 fragColor, in vec2 fragCoord ) {
    vec2 uv = fragCoord;
    float d = length(uv);
    float m = abs(sin(data.time))*0.2/d;
    vec3 col = vec3(m);
    fragColor = vec4(col, 1.0);
}

void main() {
    vec2 fragCoord = inputs.position.xy;
    vec4 fragColor;

    //calculate_color(fragColor, fragCoord);
    fragColor = inputs.color;
    fragColor *= texture(iTexture, inputs.textureCoord);

    oColor = fragColor;
}
