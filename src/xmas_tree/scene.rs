use cgmath::Point3;
use web_sys::WebGl2RenderingContext as GL;

use crate::camera::Camera;
use crate::coords::SphericalPoint3;
use crate::model::Model;
use crate::shader::Shader;
use crate::xmas_tree::ground::Ground;

pub struct Scene {
    pub camera: Camera,
    shader: Shader,
    models: Vec<Box<dyn Model>>,
}

impl Scene {
    pub fn setup(gl: &GL) -> Self {
        let camera = Camera::new(gl, SphericalPoint3::new(18., 1.7, 0.9), Point3::new(0., -1., 0.));
        let shader = Shader::new(gl);
        let mut models: Vec<Box<dyn Model>> = Vec::new();
        models.push(Box::new(Ground::new(gl)));
        Scene { camera, shader, models }
    }

    pub fn draw(&mut self, gl: &GL) {
        gl.clear_color(0., 0., 0., 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        gl.use_program(Some(&self.shader.program));

        for d in &mut self.models {
            d.draw(gl, &self.shader);
        }
    }
}
