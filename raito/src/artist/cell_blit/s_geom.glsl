#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;
out vec2 uv;

uniform vec2 view_size;
uniform vec2 cell_size;

void main() {
    vec2 diag_size = cell_size / view_size * vec2(2, -2);
    vec4 pos = gl_in[0].gl_Position;
    float id = gl_in[0].gl_PointSize;
    vec2 root = vec2(0, id * cell_size.y);

    gl_Position = pos;
    uv = root;
    EmitVertex();

    gl_Position = pos + vec4(diag_size.x, 0, 0, 0);
    uv = root + vec2(cell_size.x, 0);
    EmitVertex();

    gl_Position = pos + vec4(0, diag_size.y, 0, 0);
    uv = root + vec2(0, cell_size.y);
    EmitVertex();

    gl_Position = pos + vec4(diag_size, 0, 0);
    uv = root + cell_size;
    EmitVertex();

    EndPrimitive();
}
