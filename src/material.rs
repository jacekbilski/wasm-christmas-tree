use cgmath::{Vector3, Vector4};
use wasm_bindgen::__rt::core::mem;
use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer};

use crate::shader::MATERIALS_UBO_BINDING_POINT;

const MAX_MATERIALS: i32 = 100;

pub type MaterialId = f32;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl Material {
    fn size() -> i32 {
        let vector3_size = mem::size_of::<Vector4<f32>>() as i32; // there's no mistake, Vector3 takes the same amount of memory as Vector4
        3 * vector3_size
    }
}

pub struct Materials {
    ubo: WebGlBuffer,
    materials: Vec<Material>,
}

impl Materials {
    pub fn setup(gl: &GL) -> Self {
        Materials { ubo: Materials::setup_lights_ubo(gl), materials: vec![] }
    }

    fn setup_lights_ubo(gl: &GL) -> WebGlBuffer {
        let materials_ubo = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::UNIFORM_BUFFER, Some(&materials_ubo));
        gl.buffer_data_with_i32(GL::UNIFORM_BUFFER, MAX_MATERIALS * Material::size(), GL::STATIC_DRAW);
        gl.bind_buffer_base(GL::UNIFORM_BUFFER, MATERIALS_UBO_BINDING_POINT, Some(&materials_ubo));
        gl.bind_buffer(GL::UNIFORM_BUFFER, None);
        materials_ubo
    }

    pub fn add(&mut self, gl: &GL, material: Material) -> MaterialId {
        self.materials.push(material);
        let material_id = self.materials.len() as usize - 1;
        let vector3_size = mem::size_of::<Vector4<f32>>() as i32;

        gl.bind_buffer(GL::UNIFORM_BUFFER, Some(&self.ubo));

        unsafe {
            let array: [f32; 3] = self.materials[material_id].ambient.into();
            let js_array = js_sys::Float32Array::view(&array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, material_id as i32 * Material::size() + 0 * vector3_size, &js_array);
        }

        unsafe {
            let array: [f32; 3] = self.materials[material_id].diffuse.into();
            let js_array = js_sys::Float32Array::view(&array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, material_id as i32 * Material::size() + 1 * vector3_size, &js_array);
        }

        unsafe {
            let array: [f32; 3] = self.materials[material_id].specular.into();
            let js_array = js_sys::Float32Array::view(&array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, material_id as i32 * Material::size() + 2 * vector3_size, &js_array);
        }

        unsafe {
            let array: [f32; 1] = [self.materials[material_id].shininess];
            let js_array = js_sys::Float32Array::view(&array);
            // small hack here, shininess is not passed as a separate value, but as specular.w, 4th value in vec4
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, material_id as i32 * Material::size() + 2 * vector3_size + 12, &js_array);
        }

        gl.bind_buffer(GL::UNIFORM_BUFFER, None);

        material_id as MaterialId
    }
}
