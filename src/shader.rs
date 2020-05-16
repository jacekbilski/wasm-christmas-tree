use web_sys::{WebGl2RenderingContext as GL, WebGlProgram};
use web_sys::console;

pub const CAMERA_UBO_BINDING_POINT: u32 = 0;
pub const LIGHTS_UBO_BINDING_POINT: u32 = 1;

const VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;

layout (location = 0) in vec3 aPos;

layout (std140) uniform Camera {
    vec3 cameraPosition;
    mat4 view;
    mat4 projection;
};

void main() {
    vec4 pos = vec4(aPos, 1.0);
    gl_Position = projection * view * pos;
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

layout (std140) uniform Camera {
    vec3 cameraPosition;
    mat4 view;
    mat4 projection;
};

layout (std140) uniform Lights {
    int lightsNo;
    Light light[4];
};

out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
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
}
