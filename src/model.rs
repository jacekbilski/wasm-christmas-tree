use web_sys::WebGl2RenderingContext as GL;

use crate::shader::Shader;

pub trait Model {
    /// Do all necessary things to advance the model to the next frame
    fn next_frame(&mut self);

    /// Draw the model using given shader
    fn draw(&mut self, gl: &GL, shader: &Shader);
}
