#version 330 core

layout(location = 0) in vec4 pos;
out float id;

const mat4 transform = mat4(2, 0, 0, 0, 0, -2, 0, 0, 0, 0, 1, 0, -1, 1, 0, 1);

uniform vec2 view_size;
uniform vec2 cell_size;

void main() {
    vec3 aux_pos = vec3(pos) / vec3(view_size, 1) * vec3(cell_size, 1);
    gl_Position = transform * vec4(aux_pos, 1.0);
    gl_PointSize = pos.w;
}
