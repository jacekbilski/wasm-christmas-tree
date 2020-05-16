use web_sys::WebGl2RenderingContext as GL;

pub fn draw_triangle(gl: &GL) {
    let vertices: [f32; 9] = [
        -0.7, -0.7, 0.0,
        0.7, -0.7, 0.0,
        0.0, 0.7, 0.0,
    ];

    let buffer = gl
        .create_buffer()
        .expect("failed to create buffer");
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &vert_array,
            GL::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.draw_arrays(
        GL::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
}
