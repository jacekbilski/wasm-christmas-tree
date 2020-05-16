use web_sys::{WebGl2RenderingContext as GL, WebGlProgram};
use web_sys::console;

pub const CAMERA_UBO_BINDING_POINT: u32 = 0;
pub const LIGHTS_UBO_BINDING_POINT: u32 = 1;
pub const MATERIALS_UBO_BINDING_POINT: u32 = 2;

const VERTEX_SHADER: &str = r#"#version 300 es
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
"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
precision highp int;

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec4 specular;
};

in vec3 FragPosition;
in vec3 Normal;
flat in uint MaterialId;

layout (std140) uniform Camera {
    vec3 cameraPosition;
    mat4 view;
    mat4 projection;
};

layout (std140) uniform Lights {
    int lightsNo;
    Light light[4];
};

layout (std140) uniform Materials {
    Material material[100];
};

out vec4 FragColor;

vec3 calcLight(Light light);

void main() {
    vec3 result = vec3(0.0);
    for (int i = 0; i < lightsNo; i++) {
        result += calcLight(light[i]);
    }
    FragColor = vec4(result, 1.0);
}

vec3 calcLight(Light light) {
    vec3 ambient = light.ambient * material[MaterialId].ambient;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPosition);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light.diffuse * material[MaterialId].diffuse;

    vec3 viewDir = normalize(cameraPosition - FragPosition);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfwayDir), 0.0), material[MaterialId].specular.w);
    vec3 specular = spec * light.specular * vec3(material[MaterialId].specular);

    return ambient + diffuse + specular;
}
"#;

pub struct Shader {
    pub program: WebGlProgram,
}

impl Shader {
    pub fn new(gl: &GL) -> Shader {
        let program = gl
            .create_program()
            .expect("Cannot create program");
        let shader = Shader { program };
        shader.add_vertex_shader(&gl);
        shader.add_fragment_shader(&gl);
        gl.link_program(&shader.program);

        let success = gl
            .get_program_parameter(&shader.program, GL::LINK_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast linking result status to boolean");
        if !success {
            let message = gl.get_program_info_log(&shader.program)
                .expect("Cannot get info log");
            panic!(message);
        }

        shader.bind_camera_ubo(gl);
        shader.bind_lights_ubo(gl);
        shader.bind_materials_ubo(gl);

        shader
    }

    fn add_vertex_shader(&self, gl: &GL) {
        let shader = gl
            .create_shader(GL::VERTEX_SHADER)
            .expect("Unable to create vertex shader");
        gl.shader_source(&shader, VERTEX_SHADER);
        gl.compile_shader(&shader);
        let success = gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast compilation result status to boolean");
        if !success {
            let message = gl.get_shader_info_log(&shader)
                .expect("Cannot get info log");
            console::log_2(&"Houston, problem with vertex shader: ".into(), &message.into());
            panic!();
        }
        gl.attach_shader(&self.program, &shader);
    }

    fn add_fragment_shader(&self, gl: &GL) {
        let shader = gl
            .create_shader(GL::FRAGMENT_SHADER)
            .expect("Unable to create fragment shader");
        gl.shader_source(&shader, FRAGMENT_SHADER);
        gl.compile_shader(&shader);
        let success = gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast compilation result status to boolean");
        if !success {
            let message = gl.get_shader_info_log(&shader)
                .expect("Cannot get info log");
            console::log_2(&"Houston, problem with fragment shader: ".into(), &message.into());
            panic!();
        }
        gl.attach_shader(&self.program, &shader);
    }

    fn bind_camera_ubo(&self, gl: &GL) {
        let uniform_block_index = gl.get_uniform_block_index(&self.program, "Camera");
        gl.uniform_block_binding(&self.program, uniform_block_index, CAMERA_UBO_BINDING_POINT);
    }

    fn bind_lights_ubo(&self, gl: &GL) {
        let uniform_block_index = gl.get_uniform_block_index(&self.program, "Lights");
        gl.uniform_block_binding(&self.program, uniform_block_index, LIGHTS_UBO_BINDING_POINT);
    }

    fn bind_materials_ubo(&self, gl: &GL) {
        let uniform_block_index = gl.get_uniform_block_index(&self.program, "Materials");
        gl.uniform_block_binding(&self.program, uniform_block_index, MATERIALS_UBO_BINDING_POINT);
    }
}
