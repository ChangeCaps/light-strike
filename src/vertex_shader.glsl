#version 450

in vec3 position;

void main() {
    gl_Position = vec4(position.xy, position.z, 1.0);
}
