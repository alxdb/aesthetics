#version 150

in vec4 pos;
in vec4 nrm;
in vec2 tex;

out vec3 v_col;
out vec4 v_nrm;
out vec4 v_pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
  gl_Position = projection * view * model * pos;
  v_col = vec3(1.0, 1.0, 1.0);
  v_nrm = nrm;
  v_pos = model * pos;
}
