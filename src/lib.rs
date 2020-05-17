extern crate console_error_panic_hook;

use std::panic;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;

use crate::xmas_tree::scene::Scene;

mod camera;
mod coords;
mod lights;
mod material;
mod mesh;
mod model;
mod shader;
mod xmas_tree;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let gl = get_context();
    gl.enable(GL::DEPTH_TEST);

    let mut scene = Scene::setup(&gl);
    scene.draw(&gl);

    Ok(())
}

fn get_context() -> GL {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("Counldn't find canvas element");

    canvas
        .get_context("webgl2")
        .expect("Error getting WebGL2 Rendering Context")
        .unwrap()
        .dyn_into::<GL>()
        .expect("Error casting")
}
