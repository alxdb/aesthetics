#version 150

in vec4 pos;

out vec3 v_col;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
	v_col = vec3(0.0, 0.2, 0.5);
	gl_Position = projection * view * model * pos;
}
