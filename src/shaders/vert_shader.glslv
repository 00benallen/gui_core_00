#version 150 core

in vec2 a_Pos;
in vec2 a_Uv;

uniform Transform {
    mat4 u_Transform;
};

out vec2 v_Uv;

void main() {
    v_Uv = a_Uv;

    vec4 raw_position = vec4(a_Pos, 0.0, 1.0);

    gl_Position = raw_position * u_Transform;
}