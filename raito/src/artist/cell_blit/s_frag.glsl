#version 330 core
out vec4 fragment;
in vec2 uv;

uniform vec3 color;
uniform sampler2DRect tex;

void main() {
    float tex_value = texture(tex, uv).r;
    fragment = vec4(color * tex_value, 1);
}
