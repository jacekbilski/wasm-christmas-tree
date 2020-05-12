use web_sys::{WebGlProgram, WebGlRenderingContext};

pub struct Shader {
    pub program: WebGlProgram,
}

impl Shader {
    pub fn new(context: &WebGlRenderingContext) -> Shader {
        let program = context
            .create_program()
            .expect("Cannot create program");
        let shader = Shader { program };
        shader.add_vertex_shader(&context);
        shader.add_fragment_shader(&context);
        context.link_program(&shader.program);

        let success = context
            .get_program_parameter(&shader.program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast linking result status to boolean");
        if !success {
            let message = context.get_program_info_log(&shader.program)
                .expect("Cannot get info log");
            panic!(message);
        }

        shader
    }

    fn add_vertex_shader(&self, context: &WebGlRenderingContext) {
        let shader_source = r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#;
        let shader = context
            .create_shader(WebGlRenderingContext::VERTEX_SHADER)
            .expect("Unable to create vertex shader");
        context.shader_source(&shader, shader_source);
        context.compile_shader(&shader);
        let success = context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast compilation result status to boolean");
        if !success {
            let message = context.get_shader_info_log(&shader)
                .expect("Cannot get info log");
            panic!(message);
        }
        context.attach_shader(&self.program, &shader);
    }

    fn add_fragment_shader(&self, context: &WebGlRenderingContext) {
        let shader_source = r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;
        let shader = context
            .create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
            .expect("Unable to create fragment shader");
        context.shader_source(&shader, shader_source);
        context.compile_shader(&shader);
        let success = context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast compilation result status to boolean");
        if !success {
            let message = context.get_shader_info_log(&shader)
                .expect("Cannot get info log");
            panic!(message);
        }
        context.attach_shader(&self.program, &shader);
    }
}