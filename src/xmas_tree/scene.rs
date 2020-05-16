use cgmath::Point3;
use web_sys::WebGl2RenderingContext as GL;

use crate::camera::Camera;
use crate::coords::SphericalPoint3;
use crate::shader::Shader;
use crate::triangle;

pub struct Scene {
    pub camera: Camera,
    shader: Shader,
}

impl Scene {
    pub fn setup(gl: &GL) -> Self {
        let camera = Camera::new(SphericalPoint3::new(18., 1.7, 0.9).into(), Point3::new(0., -1., 0.), gl);
        let shader = Shader::new(gl);
        Scene { camera, shader }
    }

    pub fn draw(&mut self, gl: &GL) {
        gl.clear_color(0., 0., 0., 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        gl.use_program(Some(&self.shader.program));

        triangle::draw_triangle(gl);
    }
}
