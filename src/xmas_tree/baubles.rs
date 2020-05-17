use core::f32::consts::PI;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8};
use std::iter::FromIterator;

use cgmath::{Matrix4, Point3, vec3, Vector3};
use web_sys::WebGl2RenderingContext as GL;

use crate::coords::CylindricalPoint3;
use crate::material::{Material, MaterialId, Materials};
use crate::mesh::{Mesh, Vertex};
use crate::model::{Instance, Model};
use crate::shader::Shader;

struct Bauble {
    center: CylindricalPoint3<f32>,
    material_id: MaterialId,
}

pub struct Baubles {
    mesh: Mesh,
    baubles: Vec<Bauble>,
}

impl Baubles {
    pub fn new(gl: &GL, materials: &mut Materials) -> Self {
        let precision = 8 as u32;
        let radius = 0.2 as f32;

        let ambient: Vector3<f32> = vec3(0.1745, 0.01175, 0.01175);
        let diffuse: Vector3<f32> = vec3(0.61424, 0.04136, 0.04136);
        let specular: Vector3<f32> = vec3(0.727811, 0.626959, 0.626959);
        let shininess: f32 = 76.8;
        let red = Material { ambient, diffuse, specular, shininess };
        let red_id = materials.add(gl, red);

        let ambient: Vector3<f32> = vec3(0.01175, 0.01175, 0.1745);
        let diffuse: Vector3<f32> = vec3(0.04136, 0.04136, 0.61424);
        let specular: Vector3<f32> = vec3(0.626959, 0.626959, 0.61424);
        let shininess: f32 = 76.8;
        let blue = Material { ambient, diffuse, specular, shininess };
        let blue_id = materials.add(gl, blue);

        let ambient: Vector3<f32> = vec3(0.1745, 0.1745, 0.01175);
        let diffuse: Vector3<f32> = vec3(0.61424, 0.61424, 0.04136);
        let specular: Vector3<f32> = vec3(0.727811, 0.727811, 0.626959);
        let shininess: f32 = 76.8;
        let yellow = Material { ambient, diffuse, specular, shininess };
        let yellow_id = materials.add(gl, yellow);

        let ambient: Vector3<f32> = vec3(0.01175, 0.1745, 0.1745);
        let diffuse: Vector3<f32> = vec3(0.04136, 0.61424, 0.61424);
        let specular: Vector3<f32> = vec3(0.626959, 0.727811, 0.727811);
        let shininess: f32 = 76.8;
        let light_blue = Material { ambient, diffuse, specular, shininess };
        let light_blue_id = materials.add(gl, light_blue);

        let ambient: Vector3<f32> = vec3(0.1745, 0.01175, 0.1745);
        let diffuse: Vector3<f32> = vec3(0.61424, 0.04136, 0.61424);
        let specular: Vector3<f32> = vec3(0.727811, 0.626959, 0.727811);
        let shininess: f32 = 76.8;
        let violet = Material { ambient, diffuse, specular, shininess };
        let violet_id = materials.add(gl, violet);

        let baubles: Vec<Bauble> = vec![
            Bauble { center: CylindricalPoint3::new(0., 0., 2.7), material_id: red_id },
            Bauble { center: CylindricalPoint3::new(1.1, -0.5, 1.3), material_id: blue_id },
            Bauble { center: CylindricalPoint3::new(1.1, 1.7, 1.3), material_id: yellow_id },
            Bauble { center: CylindricalPoint3::new(1.5, 1.2, 0.25), material_id: red_id },
            Bauble { center: CylindricalPoint3::new(1.5, -1.7, 0.25), material_id: light_blue_id },
            Bauble { center: CylindricalPoint3::new(2.2, 1.0, -0.85), material_id: light_blue_id },
            Bauble { center: CylindricalPoint3::new(2.2, 3. * FRAC_PI_4, -0.85), material_id: blue_id },
            Bauble { center: CylindricalPoint3::new(2.2, -0.2, -0.85), material_id: red_id },
            Bauble { center: CylindricalPoint3::new(3., FRAC_PI_2, -1.8), material_id: violet_id },
            Bauble { center: CylindricalPoint3::new(3., -FRAC_PI_2, -1.8), material_id: yellow_id },
            Bauble { center: CylindricalPoint3::new(3., -FRAC_PI_4 - 3., -1.8), material_id: red_id },
            Bauble { center: CylindricalPoint3::new(3., 3.6, -1.8), material_id: violet_id },
            Bauble { center: CylindricalPoint3::new(3., 0.2, -1.8), material_id: blue_id },
            Bauble { center: CylindricalPoint3::new(3.6, 1. * FRAC_PI_6, -3.), material_id: light_blue_id },
            Bauble { center: CylindricalPoint3::new(3.6, 2. * FRAC_PI_6, -3.), material_id: red_id },
            Bauble { center: CylindricalPoint3::new(3.6, 4. * FRAC_PI_6, -3.), material_id: blue_id },
            Bauble { center: CylindricalPoint3::new(3.6, 5. * FRAC_PI_6, -3.), material_id: violet_id },
            Bauble { center: CylindricalPoint3::new(3.6, 6. * FRAC_PI_6, -3.), material_id: yellow_id },
            Bauble { center: CylindricalPoint3::new(3.6, 8. * FRAC_PI_6, -3.), material_id: blue_id },
            Bauble { center: CylindricalPoint3::new(3.6, 9. * FRAC_PI_6, -3.), material_id: light_blue_id },
            Bauble { center: CylindricalPoint3::new(3.6, 11. * FRAC_PI_6, -3.), material_id: yellow_id },
            Bauble { center: CylindricalPoint3::new(4., 3. * FRAC_PI_8, -4.1), material_id: light_blue_id },
            Bauble { center: CylindricalPoint3::new(4., 4. * FRAC_PI_8, -4.1), material_id: yellow_id },
            Bauble { center: CylindricalPoint3::new(4., 5. * FRAC_PI_8, -4.1), material_id: blue_id },
            Bauble { center: CylindricalPoint3::new(4., 7. * FRAC_PI_8, -4.1), material_id: violet_id },
            Bauble { center: CylindricalPoint3::new(4., 11. * FRAC_PI_8, -4.1), material_id: red_id },
            Bauble { center: CylindricalPoint3::new(4., 12. * FRAC_PI_8, -4.1), material_id: blue_id },
            Bauble { center: CylindricalPoint3::new(4., 13. * FRAC_PI_8, -4.1), material_id: yellow_id },
            Bauble { center: CylindricalPoint3::new(4., 17. * FRAC_PI_8, -4.1), material_id: red_id },
            Bauble { center: CylindricalPoint3::new(4., 21. * FRAC_PI_8, -4.1), material_id: blue_id },
        ];

        let mut vertices: Vec<Vertex> = Vec::with_capacity(2 * precision.pow(2) as usize);
        let mut indices: Vec<u32> = Vec::with_capacity(3 * 4 * precision.pow(2) as usize);

        Self::gen_sphere(&mut vertices, &mut indices, Point3::new(0., 0., 0.), radius, precision);

        let mesh = Mesh::new(gl, vertices, indices, baubles.len());

        let instances = Vec::from_iter(
            baubles.iter()
                .map(|b| {
                    let center_cartesian: Point3<f32> = b.center.into();
                    let center_arr: [f32; 3] = center_cartesian.into();
                    Instance { model: Matrix4::from_translation(Vector3::from(center_arr)), material_id : b.material_id }
                })
        );
        mesh.fill_instances_vbo(gl, &instances);
        Self { mesh, baubles }
    }

    fn gen_sphere(vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>, center: Point3<f32>, radius: f32, precision: u32) {
        Self::gen_vertices(vertices, center, radius, precision);
        Self::gen_indices(indices, precision)
    }

    fn gen_vertices(vertices: &mut Vec<Vertex>, center: Point3<f32>, radius: f32, precision: u32) {
        let angle_diff = PI / precision as f32;

        vertices.push(Vertex { position: Point3::new(center.x, center.y + radius, center.z), normal: vec3(0., 1., 0.) });

        for layer in 1..precision {
            let v_angle = angle_diff * layer as f32;   // vertically I'm doing only half rotation
            for slice in 0..(2 * precision) {
                let h_angle = angle_diff * slice as f32;   // horizontally I'm doing full circle
                let layer_radius = radius * v_angle.sin();
                let vertex = Point3::new(center.x + layer_radius * h_angle.sin(), center.y + radius * v_angle.cos(), center.z + layer_radius * h_angle.cos());

                vertices.push(Vertex { position: vertex, normal: vec3(h_angle.sin(), v_angle.cos(), h_angle.cos()) });
            }
        }

        vertices.push(Vertex { position: Point3::new(center.x, center.y - radius, center.z), normal: vec3(0., -1., 0.) });
    }

    fn gen_indices(indices: &mut Vec<u32>, precision: u32) {
        let find_index = |layer: u32, slice: u32| {
            // layers 0 and precision have only 1 vertex
            if layer == 0 {
                0
            } else if layer == precision {
                (layer - 1) * 2 * precision + 1
            } else {
                (layer - 1) * 2 * precision + 1 + slice % (2 * precision)
            }
        };

        // I'm generating indices for triangles drawn between this and previous layers of vertices
        let mut layer = 1;
        for slice in 0..2 * precision {
            // first layer has only triangles
            indices.extend([find_index(layer - 1, slice), find_index(layer, slice + 1), find_index(layer, slice)].iter());
        }

        for layer in 2..precision {
            for slice in 0..2 * precision {
                // midddle layers are actually traapezoids, I need two triangles per slice
                indices.extend([find_index(layer - 1, slice), find_index(layer, slice + 1), find_index(layer, slice)].iter());
                indices.extend([find_index(layer - 1, slice), find_index(layer - 1, slice + 1), find_index(layer, slice + 1)].iter());
            }
        }

        layer = precision;
        for slice in 0..2 * precision {
            // last layer has only triangles
            indices.extend([find_index(layer - 1, slice), find_index(layer - 1, slice + 1), find_index(layer, slice)].iter());
        }
    }
}

impl Model for Baubles {
    fn next_frame(&mut self, _gl: &GL) {
        // nothing changes
    }

    fn draw(&self, gl: &GL, shader: &Shader) {
        self.mesh.draw_instances(gl, shader, self.baubles.len());
    }
}
