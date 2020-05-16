use cgmath::Matrix4;
use wasm_bindgen::__rt::core::mem;
use web_sys::WebGl2RenderingContext as GL;

use crate::material::MaterialId;
use crate::shader::Shader;

#[derive(Debug)]
#[repr(C)]  // to make sure memory representation is like in the code
pub struct Instance {
    pub model: Matrix4<f32>,
    pub material_id: MaterialId,
}

impl Instance {
    pub fn size() -> i32 {
        (mem::size_of::<Matrix4<f32>>() + mem::size_of::<u32>()) as i32
    }

    pub fn as_vec(&self) -> Vec<f32> {
        let model: &[f32; 16] = self.model.as_ref();
        let material: [f32; 1] = [self.material_id];
        [&model[..], &material[..]].concat()
    }
}

pub trait Model {
    /// Do all necessary things to advance the model to the next frame
    fn next_frame(&mut self);

    /// Draw the model using given shader
    fn draw(&mut self, gl: &GL, shader: &Shader);
}
