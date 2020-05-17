use cgmath::{Matrix4, Point3, vec3, Vector3};
use tobj::{load_mtl_buf, load_obj_buf};
use wasm_bindgen::__rt::std::io::BufReader;
use web_sys::WebGl2RenderingContext as GL;

use crate::material::{Material, Materials};
use crate::mesh::{Mesh, Vertex};
use crate::model::{Instance, Model};
use crate::shader::Shader;

static TREE_MODEL: &'static str = include_str!("../../models/tree.obj");
static TREE_MATERIALS: &'static str = include_str!("../../models/tree.mtl");

pub struct Tree {
    meshes: Vec<Mesh>,
}

impl Tree {
    pub fn new(gl: &GL, materials: &mut Materials) -> Self {
        Self::from_model(gl, materials)
    }

    fn from_model(gl: &GL, materials: &mut Materials) -> Self {
        let mut model_reader = BufReader::new(TREE_MODEL.as_bytes());
        let tree = load_obj_buf(&mut model_reader, false, |_p| load_mtl_buf(&mut BufReader::new(TREE_MATERIALS.as_bytes())));
        let (models, model_materials) = tree.unwrap();
        let mut meshes: Vec<Mesh> = vec![];
        for mi in 0..models.len() {
            let mut vertices: Vec<Vertex> = vec![];
            let mut indices: Vec<u32> = vec![];
            let mesh = models[mi].mesh.clone();
            for vi in (0..mesh.positions.len()).step_by(3) {
                let position = Point3::new(mesh.positions[vi], mesh.positions[vi+1], mesh.positions[vi+2]);
                let normal = vec3(mesh.normals[vi], mesh.normals[vi+1], mesh.normals[vi+2]);
                vertices.push(Vertex { position, normal });
            }
            indices.extend(mesh.indices.iter());

            let material = &model_materials[models[mi].mesh.material_id.unwrap()];
            let my_material = Material{ambient: Vector3::from(material.ambient), diffuse: Vector3::from(material.diffuse), specular: Vector3::from(material.specular), shininess: material.shininess};
            let material_id = materials.add(gl, my_material);
            let mesh = Mesh::new(gl, vertices, indices, 1);
            let scaling = Matrix4::from_nonuniform_scale(1.8, 1., 1.8);
            mesh.fill_instances_vbo(gl, &vec![Instance { model: scaling, material_id }]);
            meshes.push(mesh);
        }

        Self { meshes }
    }
}

impl Model for Tree {
    fn next_frame(&mut self, _gl: &GL) {
        // nothing changes
    }

    fn draw(&self, gl: &GL, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw_single(gl, shader);
        }
    }
}
