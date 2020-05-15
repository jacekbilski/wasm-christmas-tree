use web_sys::{WebGl2RenderingContext, WebGlProgram};
use web_sys::console;

const VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
in vec4 position;
void main() {
    gl_Position = position;
}
"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

pub struct Shader {
    pub program: WebGlProgram,
}

impl Shader {
    pub fn new(context: &WebGl2RenderingContext) -> Shader {
        let program = context
            .create_program()
            .expect("Cannot create program");
        let shader = Shader { program };
        shader.add_vertex_shader(&context);
        shader.add_fragment_shader(&context);
        context.link_program(&shader.program);

        let success = context
            .get_program_parameter(&shader.program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast linking result status to boolean");
        if !success {
            let message = context.get_program_info_log(&shader.program)
                .expect("Cannot get info log");
            panic!(message);
        }

        shader
    }

    fn add_vertex_shader(&self, context: &WebGl2RenderingContext) {
        let shader = context
            .create_shader(WebGl2RenderingContext::VERTEX_SHADER)
            .expect("Unable to create vertex shader");
        context.shader_source(&shader, VERTEX_SHADER);
        context.compile_shader(&shader);
        let success = context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast compilation result status to boolean");
        if !success {
            let message = context.get_shader_info_log(&shader)
                .expect("Cannot get info log");
            console::log_2(&"Houston, problem with vertex shader: ".into(), &message.into());
            panic!();
        }
        context.attach_shader(&self.program, &shader);
    }

    fn add_fragment_shader(&self, context: &WebGl2RenderingContext) {
        let shader = context
            .create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)
            .expect("Unable to create fragment shader");
        context.shader_source(&shader, FRAGMENT_SHADER);
        context.compile_shader(&shader);
        let success = context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .expect("Kaboom, Cannot cast compilation result status to boolean");
        if !success {
            let message = context.get_shader_info_log(&shader)
                .expect("Cannot get info log");
            console::log_2(&"Houston, problem with fragment shader: ".into(), &message.into());
            panic!();
        }
        context.attach_shader(&self.program, &shader);
    }
}
