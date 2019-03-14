#version 150

in vec3 v_col;
in vec4 v_nrm;
in vec4 v_pos;

out vec4 color;

void main() {
  vec4 light_pos = vec4(0.0, 10.0, 10.0, 1.0);
  vec4 light_col = vec4(1.0, 1.0, 1.0, 1.0);
  // vec4 light_dir = normalize(light_pos - v_pos);
  vec4 light_dir = normalize(vec4(0.0, 1.0, 1.0, 0.0));
  float illum = max(dot(v_nrm, light_dir), 0.0);
  vec4 diffuse = illum * light_col;
	color = diffuse * vec4(v_col, 1.0);
}
