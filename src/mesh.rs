use cgmath::{Point3, Vector3, Vector4};
use wasm_bindgen::__rt::core::mem;
use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer, WebGlVertexArrayObject};

// use crate::model::Instance;
use crate::shader::Shader;

#[repr(C)]  // to make sure memory representation is like in the code
#[derive(Debug)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    pub fn size() -> usize {
        let float_size = 4; //mem::size_of::<GLfloat>();
        2 * 3 * float_size
    }

    pub fn as_vec(&self) -> Vec<f32> {
        let pos: [f32; 3] = self.position.into();
        let norm: [f32; 3] = self.normal.into();
        [&pos[..], &norm[..]].concat()
    }
}

pub struct Mesh {
    indices: Vec<u32>,
    // max_instances: usize,
    vao: WebGlVertexArrayObject,
    // instances_vbo: VBO,
}

impl Mesh {
    pub fn new(gl: &GL, vertices: Vec<Vertex>, indices: Vec<u32>, max_instances: usize) -> Self {
        // let instances_vbo = Self::create_instances_vbo(max_instances);
        let vao = Self::create_vao(gl, &vertices, &indices /*, instances_vbo*/);
        let mesh = Self { indices, /*max_instances,*/ vao /*, instances_vbo*/ };
        mesh
    }

    fn create_vao(gl: &GL, vertices: &Vec<Vertex>, indices: &Vec<u32> /*, instances_vbo: u32*/) -> WebGlVertexArrayObject {
        let vao = gl.create_vertex_array().unwrap(); // create VAO
        gl.bind_vertex_array(Some(&vao)); // ...and bind it

        Self::create_vbo(gl, vertices);
        Self::create_ebo(gl, indices);

        let stride = Vertex::size() as i32;
        // tell GL how to interpret the data in VBO -> one triangle vertex takes 3 coordinates (x, y, z)
        // this call also connects my VBO to this attribute
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0); // enable the attribute for position

        /*// second three floats are for normal vector
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1); // enable the attribute for colour

        // enter instancing, using completely different VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo);
        let vec4_size = mem::size_of::<Vector4<f32>>() as i32;
        let instances_stride = Instance::size() as GLsizei;
        // println!("Instances stride: {}, Instance.size: {}", instances_stride, Instance::size());

        // model matrix with rotation and translation
        // I need to do the calls below 4 times, because size can be at most 4, but I'm sending a matrix of size 16
        gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, instances_stride, ptr::null());
        gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, instances_stride, vec4_size as *const c_void);
        gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, instances_stride, (2 * vec4_size) as *const c_void);
        gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, instances_stride, (3 * vec4_size) as *const c_void);
        gl::EnableVertexAttribArray(2);
        gl::EnableVertexAttribArray(3);
        gl::EnableVertexAttribArray(4);
        gl::EnableVertexAttribArray(5);
        gl::VertexAttribDivisor(2, 1);    // every iteration
        gl::VertexAttribDivisor(3, 1);    // every iteration
        gl::VertexAttribDivisor(4, 1);    // every iteration
        gl::VertexAttribDivisor(5, 1);    // every iteration

        // material_id
        gl::VertexAttribPointer(6, 1, gl::FLOAT, gl::FALSE, instances_stride, (4 * vec4_size) as *const c_void);
        gl::EnableVertexAttribArray(6);
        gl::VertexAttribDivisor(6, 1);    // every iteration */

        gl.bind_buffer(GL::ARRAY_BUFFER, None); // unbind instances VBO
        // do NOT unbind EBO, VAO would remember that
        gl.bind_vertex_array(None); // unbind my VAO
        vao
    }

    fn create_vbo(gl: &GL, vertices: &Vec<Vertex>) {
        let vbo = gl.create_buffer().unwrap(); // create buffer for my data
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo)); // ARRAY_BUFFER now "points" to my buffer
        unsafe {
            let vec: Vec<f32> = vertices.iter().flat_map(|v| {let vec: Vec<f32> = v.as_vec(); vec}).collect();
            let js_array = js_sys::Float32Array::view(&vec);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_array, GL::STATIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
        }
    }

    fn create_ebo(gl: &GL, indices: &[u32]) {
        let ebo = gl.create_buffer().unwrap(); // create buffer for indices (elements)
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ebo)); // ELEMENT_ARRAY_BUFFER now "points" to my buffer
        unsafe {
            let js_array = js_sys::Uint32Array::view(&indices);
            gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &js_array, GL::STATIC_DRAW); // actually fill ELEMENT_ARRAY_BUFFER with data
        }
    }

    /*pub fn fill_instances_vbo(&self, instances: &Vec<Instance>) {
        // println!("Instance[0]: {:?}", instances[0]);
        // println!("Instance: {:?}", instances);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ARRAY_BUFFER,
                           (self.max_instances * Instance::size()) as GLsizeiptr,
                           instances.as_ptr() as *const c_void,
                           gl::DYNAMIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
        }
    }

    fn create_instances_vbo(max_instances: usize) -> VBO {
        unsafe {
            let mut instances_vbo = 0 as VBO;
            gl::GenBuffers(1, &mut instances_vbo); // create buffer for my data
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ARRAY_BUFFER,
                           (max_instances * Instance::size()) as GLsizeiptr,
                           ptr::null(), // don't fill, only reserve space
                           gl::DYNAMIC_DRAW);
            instances_vbo
        }
    }*/

    pub fn draw_single(&self, gl: &GL, shader: &Shader) {
        gl.use_program(Some(&shader.program));
        gl.bind_vertex_array(Some(&self.vao));
        gl.draw_elements_with_i32(GL::TRIANGLES, self.indices.len() as i32, GL::UNSIGNED_INT, 0);
        gl.bind_vertex_array(None);
    }

    /*pub fn draw_instances(&mut self, shader: &Shader, num: usize) {
        unsafe {
            gl::UseProgram(shader.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElementsInstanced(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null(), num as i32);
            gl::BindVertexArray(0);
        }
    }*/
}
