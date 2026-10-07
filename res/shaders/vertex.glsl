#version 460 core

in vec2 position;

out vec2 uv;

// uniform mat3 viewMatrix;
const float ar = 1920.0 / 1080.0;
const mat3 viewMatrix = transpose(mat3(
    2.0, 0.0, -1.0,
    0.0, 2.0, -1.0,
    0.0, 0.0, 1.0
));

void main(void) {
    vec2 screenPos = (viewMatrix * vec3(position, 1.0)).xy;
    gl_Position = vec4(screenPos, 0.0, 1.0);
    uv = vec2(position.x * ar, position.y);
}
