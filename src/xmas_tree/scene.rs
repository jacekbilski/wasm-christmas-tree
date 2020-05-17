use cgmath::{Point3, vec3};
use web_sys::WebGl2RenderingContext as GL;

use crate::camera::Camera;
use crate::coords::SphericalPoint3;
use crate::lights::Lights;
use crate::material::Materials;
use crate::model::Model;
use crate::shader::Shader;
use crate::xmas_tree::baubles::Baubles;
use crate::xmas_tree::ground::Ground;
use crate::xmas_tree::snow::Snow;

pub struct Scene {
    pub camera: Camera,
    lights: Lights,
    shader: Shader,
    models: Vec<Box<dyn Model>>,
}

impl Scene {
    pub fn setup(gl: &GL) -> Self {
        let camera = Camera::new(gl, SphericalPoint3::new(18., 1.7, 0.9), Point3::new(0., -1., 0.));
        let mut lights = Lights::setup(gl);
        lights.add(gl, Point3::new(10., 100., 10.), vec3(0.3, 0.3, 0.3), vec3(0.2, 0.2, 0.2), vec3(0., 0., 0.));
        lights.add(gl, Point3::new(5., 6., 2.), vec3(0.2, 0.2, 0.2), vec3(2., 2., 2.), vec3(0.5, 0.5, 0.5));

        let shader = Shader::new(gl);

        let mut materials = Materials::setup(gl);
        let models = Scene::add_models(gl, &mut materials);
        Scene { camera, lights, shader, models }
    }

    fn add_models(gl: &GL, materials: &mut Materials) -> Vec<Box<dyn Model>> {
        let mut models: Vec<Box<dyn Model>> = Vec::new();
        models.push(Box::new(Ground::new(gl, materials)));
        // models.push(Box::new(Tree::new(materials)));
        models.push(Box::new(Baubles::new(gl, materials)));
        models.push(Box::new(Snow::new(gl, materials)));
        models
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
