use cgmath::{Deg, Matrix4, perspective, Point3, vec3, Vector4};
use wasm_bindgen::__rt::core::mem;
use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer};

use crate::coords::SphericalPoint3;
use crate::shader::CAMERA_UBO_BINDING_POINT;

pub struct Camera {
    position: SphericalPoint3<f32>,
    look_at: Point3<f32>,
    ubo: WebGlBuffer,
    window_width: f32,
    window_height: f32,
}

impl Camera {
    pub fn new(position: SphericalPoint3<f32>, look_at: Point3<f32>, gl: &GL) -> Self {
        let (window_width, window_height) = (gl.drawing_buffer_width(), gl.drawing_buffer_height());
        let ubo = Camera::setup_camera_ubo(&gl);
        let camera = Camera { position, look_at, ubo, window_width: window_width as f32, window_height: window_height as f32 };
        camera.update_uniforms(gl);
        camera
    }

    fn setup_camera_ubo(gl: &GL) -> WebGlBuffer {
        let camera_ubo = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::UNIFORM_BUFFER, Some(&camera_ubo));
        let matrix_size = mem::size_of::<Matrix4<f32>>() as i32;
        let vector3_size = mem::size_of::<Vector4<f32>>() as i32; // there's no mistake, Vector3 takes the same amount of memory as Vector4
        gl.buffer_data_with_i32(GL::UNIFORM_BUFFER, vector3_size + 2 * matrix_size, GL::STATIC_DRAW);
        gl.bind_buffer_base(GL::UNIFORM_BUFFER, CAMERA_UBO_BINDING_POINT, Some(&camera_ubo));
        gl.bind_buffer(GL::UNIFORM_BUFFER, None);
        camera_ubo
    }

    fn update_uniforms(&self, gl: &GL) {
        gl.bind_buffer(GL::UNIFORM_BUFFER, Some(&self.ubo));
        let matrix_size = mem::size_of::<Matrix4<f32>>() as i32;
        let vector3_size = mem::size_of::<Vector4<f32>>() as i32; // there's no mistake, Vector3 takes the same amount of memory as Vector4
        unsafe {
            let pos: Point3<f32> = self.position.into();
            let pos_array: [f32; 3] = pos.into();
            let pos_js_array = js_sys::Float32Array::view(&pos_array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, 0, &pos_js_array);
        }

        unsafe {
            let view: Matrix4<f32> = Matrix4::look_at(self.position.into(), self.look_at, vec3(0.0, 1.0, 0.0));
            let view_array: &[f32; 16] = view.as_ref();
            let view_js_array = js_sys::Float32Array::view(view_array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, vector3_size, &view_js_array);
        }

        unsafe {
            let projection = perspective(Deg(45.0), self.window_width / self.window_height, 0.1, 100.0);
            let projection_array: &[f32; 16] = projection.as_ref();
            let projection_js_array = js_sys::Float32Array::view(projection_array);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::UNIFORM_BUFFER, vector3_size + matrix_size, &projection_js_array);
        }
        gl.bind_buffer(GL::UNIFORM_BUFFER, None);
    }
}
