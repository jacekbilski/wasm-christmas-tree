#![allow(dead_code)]

use cgmath::{Point3, Vector3, Vector4};
use wasm_bindgen::__rt::core::mem;
use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer};

use crate::shader::LIGHTS_UBO_BINDING_POINT;

const MAX_LIGHTS: i32 = 4;

struct Light {
    position: Point3<f32>,
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
}

pub struct Lights {
    ubo: WebGlBuffer,
    lights: Vec<Light>,
}

impl Lights {
    pub fn setup(gl: &GL) -> Self {
        Lights { ubo: Lights::setup_lights_ubo(gl), lights: vec![] }
    }

    fn setup_lights_ubo(gl: &GL) -> WebGlBuffer {
        let lights_ubo = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::UNIFORM_BUFFER, Some(&lights_ubo));
        let vector3_size = mem::size_of::<Vector4<f32>>() as i32; // there's no mistake, Vector3 takes the same amount of memory as Vector4
        gl.buffer_data_with_i32(GL::UNIFORM_BUFFER, 16 + MAX_LIGHTS * 4 * vector3_size, GL::STATIC_DRAW);
        gl.bind_buffer_base(GL::UNIFORM_BUFFER, LIGHTS_UBO_BINDING_POINT, Some(&lights_ubo));
        gl.bind_buffer(GL::UNIFORM_BUFFER, None);
        lights_ubo
    }

    pub fn add(&mut self, gl: &GL, position: Point3<f32>, ambient: Vector3<f32>, diffuse: Vector3<f32>, specular: Vector3<f32>) {
        let light = Light {position, ambient, diffuse, specular};
        self.lights.push(light);
        let lights_no= self.lights.len() as i32;

        let vector3_size = mem::size_of::<Vector4<f32>>() as i32;
        let light_size = 4 * vector3_size;
        gl.bind_buffer(GL::UNIFORM_BUFFER, Some(&self.ubo));

        unsafe {
            let array: [i32; 1] = [lights_no];
            let js_array = js_sys::Int32Array::view(&array[..]);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, 0, &js_array);
        }

        unsafe {
            let array: [f32; 3] = position.into();
            let js_array = js_sys::Float32Array::view(&array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 0 * vector3_size, &js_array);
        }

        unsafe {
            let array: [f32; 3] = ambient.into();
            let js_array = js_sys::Float32Array::view(&array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 1 * vector3_size, &js_array);
        }

        unsafe {
            let array: [f32; 3] = diffuse.into();
            let js_array = js_sys::Float32Array::view(&array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 2 * vector3_size, &js_array);
        }

        unsafe {
            let array: [f32; 3] = specular.into();
            let js_array = js_sys::Float32Array::view(&array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 3 * vector3_size, &js_array);
        }

        gl.bind_buffer(GL::UNIFORM_BUFFER, None);
    }
}
