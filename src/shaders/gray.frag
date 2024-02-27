#version 330 core
out vec4 FragColor;

in vec3 ourColor; // Receive color from the vertex shader

void main()
{
    // Convert to grayscale using luminosity method
    float gray = dot(ourColor, vec3(0.299, 0.587, 0.114));
    FragColor = vec4(gray, gray, gray, 1.0);
}
