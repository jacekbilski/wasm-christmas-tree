extern crate console_error_panic_hook;

use std::panic;

use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};

use crate::xmas_tree::scene::Scene;

mod camera;
mod coords;
mod lights;
mod material;
mod mesh;
mod model;
mod shader;
mod xmas_tree;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = window().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("Counldn't find canvas element");

    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);

    let gl = get_context(&canvas);
    gl.enable(GL::DEPTH_TEST);

    let mut scene = Scene::setup(&gl);

    let render_loop = Rc::new(RefCell::new(None));
    let render_loop_2 = render_loop.clone();
    *render_loop_2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        scene.next_frame(&gl);
        scene.draw(&gl);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(render_loop.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(render_loop_2.borrow().as_ref().unwrap());
    Ok(())
}

fn get_context(canvas: &HtmlCanvasElement) -> GL {
    canvas
        .get_context("webgl2")
        .expect("Error getting WebGL2 Rendering Context")
        .unwrap()
        .dyn_into::<GL>()
        .expect("Error casting")
}
