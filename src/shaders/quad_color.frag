#version 330 core
uniform vec2 resolution;

out vec4 FragColor;

void main() {
    vec2 p = gl_FragCoord.xy / resolution;
    float gray = 1.0 - p.x;
    float red = p.y;
    FragColor = vec4(red, gray * red, gray * red, 1.0);
}
