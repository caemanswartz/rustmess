#version 150
in vec3 position;
in vec3 normal;
in vec3 texture;
out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;
uniform vec3 translation;
uniform mat4 rotation;
uniform vec3 scaling;
uniform mat4 view;
uniform mat4 perspective;
void main() {
    v_tex_coords = vec2(texture.x, texture.y);
    mat4 model = mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            translation.x, translation.y, translation.z, 1.0
        ) * transpose(rotation) * mat4(
            scaling.x, 0.0, 0.0, 0.0,
            0.0, scaling.y, 0.0, 0.0,
            0.0, 0.0, scaling.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}