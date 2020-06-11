use cgmath::{Matrix4, Point3, SquareMatrix, vec3, Vector3};
use web_sys::WebGl2RenderingContext as GL;

use crate::material::{Material, Materials};
use crate::mesh::{Mesh, Vertex};
use crate::model::{Instance, Model};
use crate::shader::Shader;

pub struct Ground {
    mesh: Mesh,
}

impl Ground {
    pub fn new(gl: &GL, materials: &mut Materials) -> Self {
        let vertices: Vec<Vertex> = vec![
            Vertex { position: Point3::new(-10., -5., -10.), normal: vec3(0., 1., 0.) },   // far
            Vertex { position: Point3::new(-10., -5., 10.), normal: vec3(0., 1., 0.) }, // left
            Vertex { position: Point3::new(10., -5., -10.), normal: vec3(0., 1., 0.) }, // right
            Vertex { position: Point3::new(10., -5., 10.), normal: vec3(0., 1., 0.) }, // near
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2,
            1, 3, 2,
        ];

        let ambient: Vector3<f32> = vec3(1., 1., 1.);
        let diffuse: Vector3<f32> = vec3(0.623960, 0.686685, 0.693872);
        let specular: Vector3<f32> = vec3(0.5, 0.5, 0.5);
        let shininess: f32 = 225.;
        let material = Material { ambient, diffuse, specular, shininess };
        let material_id = materials.add(gl, material);

        let mesh = Mesh::new(gl, vertices, indices, 1);
        mesh.fill_instances_vbo(gl, &vec![Instance { model: Matrix4::identity(), material_id }]);
        Self { mesh }
    }
}

impl Model for Ground {
    fn next_frame(&mut self, _gl: &GL) {
        // nothing changes
    }

    fn draw(&self, gl: &GL, shader: &Shader) {
        self.mesh.draw_single(gl, shader);
    }
}
