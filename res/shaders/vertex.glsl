#version 460 core

in vec2 position;

out vec2 uv;

const float width = 2880;
const float height = 1920 - 82;
// const float width = 1920;
// const float height = 1080;
const mat3 viewMatrix = transpose(mat3(
    2.0, 0.0, -1.0,
    0.0, 2.0, -1.0,
    0.0, 0.0, 1.0
));

void main(void) {
    vec2 screenPos = (viewMatrix * vec3(position, 1.0)).xy;
    gl_Position = vec4(screenPos, 0.0, 1.0);
    uv = vec2(position.x / height * width, position.y);
}
