use web_sys::{WebGl2RenderingContext as GL, WebGlProgram};
use web_sys::console;

pub const CAMERA_UBO_BINDING_POINT: u32 = 0;
pub const LIGHTS_UBO_BINDING_POINT: u32 = 1;
pub const MATERIALS_UBO_BINDING_POINT: u32 = 2;

const VERTEX_SHADER: &'static str = include_str!("../shaders/standard.vert");

const FRAGMENT_SHADER: &'static str = include_str!("../shaders/standard.frag");

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
