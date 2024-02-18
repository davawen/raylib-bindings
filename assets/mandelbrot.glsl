#version 330 core
#extension GL_ARB_gpu_shader_fp64 : enable

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 color;

struct Complex {
	float real;
	float imag;
};

const int max_iter = 20;

uniform vec2 resolution;
uniform vec2 cam;

int mandelbrot(in Complex c) {
	Complex z = c;
	for (int i = 0; i < max_iter; i++) {
		if (z.real*z.real + z.imag*z.imag >= 4.0) { // euclidian distance >= 2
            return i;
       }

		// (a + bi)^2
		// a^2 + 2abi - b^2
		z = Complex(
			z.real*z.real - z.imag*z.imag,
			2.0*z.real*z.imag
		);

		z.real += c.real;
		z.imag += c.imag;
	}

    return max_iter;
}

void main() {
	vec2 p = gl_FragCoord.xy / resolution; 

	Complex z = Complex(
		(p.x - 0.5) + cam.x,
		((p.y - 0.5) * resolution.y/resolution.x) + cam.y
	);

	int i = mandelbrot(z);
	const vec3 colors[6] = vec3[6](
		vec3(0.1, 0.1, 0.3),
		vec3(0.1, 0.2, 0.9),
		vec3(0.1, 0.95, 0.95),
		vec3(0.2, 0.95, 0.1),
		vec3(0.9, 0.95, 0.1),
		vec3(0.95, 0.1, 0.1)
	);

	if (i == max_iter) color = vec4(0.0, 0.0, 0.0, 1.0);
	else {
		float range = float(i*5) / float(max_iter);

		vec3 a = colors[uint(floor(range))];
		vec3 b = colors[uint(ceil(range))];
		color = vec4(mix(a, b, fract(range)), 1.0);
	}
}

