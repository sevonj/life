shader_type spatial;
render_mode unshaded;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	ALBEDO.r = UV.x;
	ALBEDO.g = 0.0;
	ALBEDO.b = 0.0;
}

//void light() {
	// Called for every pixel for every light affecting the material.
	// Uncomment to replace the default light processing function with this one.
//}
