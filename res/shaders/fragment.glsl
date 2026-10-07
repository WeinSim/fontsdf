#version 460 core

in vec2 uv;

out vec4 outColor;

const vec2 point = vec2(0.4, 0.6);

void main(void) {
    float d = distance(point, uv);
    outColor = vec4(vec3(d), 1.0);
}
