#version 330 core

uniform float aspectRatio;
uniform vec2 selfPosition;
uniform vec2 selfScale;

layout (location = 0) in vec2 pixel;
out vec2 fragCoords;

void main () {
    vec2 screenCords = (pixel * selfScale) + selfPosition;
    screenCords.x *= aspectRatio;

    fragCoords = pixel;
    gl_Position = vec4(screenCords, 0., 1.);
}