#version 430 core

in vec3 vertexColor;
in float depth;
out vec4 FragColor;

float fog(float depth) {
    if (depth <= 1) {
        return 1;
    }

    return 1.0 / (pow(depth, 0.5));
}

void main() {
    vec3 clr = vertexColor * fog(depth);
    FragColor = vec4(clr, 1.0);
}