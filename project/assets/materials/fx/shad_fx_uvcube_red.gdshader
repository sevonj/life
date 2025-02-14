shader_type spatial;
render_mode unshaded;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	ALBEDO.r = 1.0;
	ALBEDO.g = UV.x;
	ALBEDO.b = UV.x;
}

//void light() {
	// Called for every pixel for every light affecting the material.
	// Uncomment to replace the default light processing function with this one.
//}
