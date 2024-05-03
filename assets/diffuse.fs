#version 330 core

in vec4 fragNormal;
in vec4 fragColor;
uniform vec4 colDiffuse;

// Output fragment color
out vec4 finalColor;

void main()
{
    float diff = max( dot( fragNormal.xyz, vec3(0.0, 1.0, 0.0) ), 0.0 );
    finalColor = vec4(colDiffuse.rgb * diff, colDiffuse.a);
}
