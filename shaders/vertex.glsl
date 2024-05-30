#version 430 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

out vec3 vertexColor;

uniform mat4 modelTransformMatrix;
uniform mat4 projectionMatrix;

void main() {
    vec4 v = vec4(position, 1.0);
    v = modelTransformMatrix * v;
    v = projectionMatrix * v;
    gl_Position = v;
    vertexColor = color;
}