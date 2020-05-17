#version 300 es
precision highp float;
precision highp int;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in mat4 instanceModel;
layout (location = 6) in float instanceMaterialId;

layout (std140) uniform Camera {
    vec3 cameraPosition;
    mat4 view;
    mat4 projection;
};

out vec3 FragPosition;
out vec3 Normal;
flat out uint MaterialId;

void main() {
    vec4 pos = instanceModel * vec4(aPos, 1.0);
    gl_Position = projection * view * pos;
    FragPosition = vec3(pos);
    Normal = mat3(transpose(inverse(instanceModel))) * aNormal;
    MaterialId = uint(instanceMaterialId);
}
