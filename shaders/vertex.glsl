#version 430 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

out vec3 vertexColor;
out float depth;

uniform mat4 transformMatrix;

void main() {
    vec4 v = vec4(position, 1.0);
    v = transformMatrix * v;
    gl_Position = v;
    vertexColor = color;
    depth = v.z;
}