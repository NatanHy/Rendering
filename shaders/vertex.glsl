#version 430 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 textureCoordinate;

out vec3 vertexColor;
out vec3 vertexNorm;
out vec2 texCoord;
out float depth;

uniform mat4 transformMatrix;

void main() {
    vec4 v = vec4(position, 1.0);
    v = transformMatrix * v;
    gl_Position = v;
    vertexColor = vec3(1.);
    vertexNorm = normal;
    texCoord = vec2(textureCoordinate.x, 1. - textureCoordinate.y);
    depth = v.z;
}