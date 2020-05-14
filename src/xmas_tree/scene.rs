use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::shader::Shader;
use crate::triangle;

pub struct Scene {
    shader: Shader,
}

impl Scene {
    pub fn setup(context: &WebGl2RenderingContext) -> Self {
        let shader = Shader::new(&context);
        Scene { shader }
    }

    pub fn draw(&mut self, context: &WebGl2RenderingContext) {
        context.clear_color(0., 0., 0., 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        context.use_program(Some(&self.shader.program));

        triangle::draw_triangle(context);
    }
}
