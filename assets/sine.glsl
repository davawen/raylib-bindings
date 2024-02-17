#version 330

uniform float time;

in vec2 fragTexCoord;       // Fragment input attribute: texture coordinate
in vec4 fragColor;          // Fragment input attribute: color
out vec4 finalColor;        // Fragment output: color

void main() {
	finalColor = vec4(sin(time)/2.0 + 0.5, sin(time*3.0)/2.0 + 0.5, 0.0, 1.0);
}
