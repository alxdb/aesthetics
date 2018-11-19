#version 150

in vec3 pos;

out vec3 v_col;

void main() {
	v_col = vec3(0.0, 0.2, 0.5);
	gl_Position = vec4(pos, 1.0);
}
