#version 150

in vec3 v_col;

out vec4 color;

void main() {
	color = vec4(v_col, 1.0);
}
