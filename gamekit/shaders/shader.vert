//
// Vertex Shader
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

layout (location = 0) in vec3 iPosition;
layout (location = 1) in vec4 iColor;
layout (location = 2) in vec2 iTextureCoord;

layout (location = 0) out vertex_data {
    vec4 position;
    vec4 color;
    vec2 textureCoord;
} outputs;

void main() {
    float screen_width = data.resolution_x > 0.0 ? data.resolution_x : 1.0;
    float screen_height = data.resolution_y > 0.0 ? data.resolution_y : 1.0;

    float width = data.x_max - data.y_min; if (width == 0.0) width = 1.0;
    float height = data.y_max - data.y_min; if (height == 0.0) height = 1.0;

    float x = -1.0 + 2.0 * (iPosition.x - data.x_min) / width;
    float y = 1.0 - 2.0 * (iPosition.y - data.y_min) / height;

    outputs.position = vec4(x, y, iPosition.z, 1.0);
    outputs.textureCoord = iTextureCoord;
    outputs.color = iColor;

    gl_Position = vec4(x, y, iPosition.z, 1.0);
}
