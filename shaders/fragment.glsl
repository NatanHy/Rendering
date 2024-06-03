#version 430 core

const vec3 lightDir = vec3(0.0, 1.0, 1.0);

in vec3 vertexColor;
in vec3 vertexNorm;
in vec2 texCoord;
in float depth;

out vec4 FragColor;

uniform sampler2D texture0;

float fog(float depth) {
    if (depth <= 1) {
        return 1;
    }

    return 1.0 / (pow(depth, 0.5));
}

void main() {
    float lightCoef = (1 + dot(vertexNorm, lightDir)) / 2.;
    FragColor = texture(texture0, texCoord);
}