#version 330 core
uniform vec3 color;

in vec2 fragCoords;
out vec4 fragColor;

void main () {
    if (fragCoords.x * fragCoords.x + fragCoords.y * fragCoords.y <= 1.) {
        fragColor = vec4(color, 1);
    } else {
        fragColor = vec4(0, 0, 0, 0);
    }
}