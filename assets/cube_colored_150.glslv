#version 150 core
in vec3 a_pos;
uniform mat4 u_model_view_proj;
void main() {
    gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
}
