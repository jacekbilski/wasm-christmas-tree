extern crate console_error_panic_hook;

use core::f32::consts::PI;
use std::panic;

use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlCanvasElement, MouseEvent, TouchEvent, WebGl2RenderingContext as GL};

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

fn get_canvas() -> HtmlCanvasElement {
    let document = window().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().expect("Counldn't find canvas element");
    canvas
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

struct Rotation {
    x_offset: i32,
    y_offset: i32,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let canvas = get_canvas();
    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);

    let gl = get_context(&canvas);
    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::CULL_FACE);

    let mut scene = Scene::setup(&gl);

    {   // handling mouse "dragging" - rotating the scene
        let gl = gl.clone();
        let mut camera = scene.camera.clone();
        let canvas2 = canvas.clone();
        let on_mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            if event.buttons() == 1 {
                let max = (canvas2.width() as f32).max(canvas2.height() as f32);
                camera.rotate_horizontally(&gl, -2. * PI / max * event.movement_x() as f32);
                camera.rotate_vertically(&gl, -2. * PI / max * event.movement_y() as f32);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.set_onmousemove(Some(on_mouse_move.as_ref().unchecked_ref()));
        on_mouse_move.forget();
    }

    {   // handling touch "dragging" - rotating the scene
        let gl = gl.clone();
        let mut camera = scene.camera.clone();
        let canvas2 = canvas.clone();
        let mut rotation = Rotation { x_offset: 0, y_offset: 0 };
        let on_touch_move = Closure::wrap(Box::new(move |event: TouchEvent| {
            let ev: Event = event.clone().into();
            // I'm only supporting single/first touch
            let first = event.touches().get(0).unwrap();
            if ev.type_() == "touchstart" {
                rotation.x_offset = first.page_x();
                rotation.y_offset = first.page_y();
            }
            if ev.type_() == "touchmove" {
                let max = (canvas2.width() as f32).max(canvas2.height() as f32);
                camera.rotate_horizontally(&gl, -2. * PI / max * (first.page_x() - rotation.x_offset) as f32);
                camera.rotate_vertically(&gl, 2. * PI / max * (first.page_y() - rotation.y_offset) as f32);
                rotation.x_offset = first.page_x();
                rotation.y_offset = first.page_y();
            }
        }) as Box<dyn FnMut(_)>);
        canvas.set_ontouchmove(Some(on_touch_move.as_ref().unchecked_ref()));
        on_touch_move.forget();
    }

    {   // handling resizing the canvas
        let gl = gl.clone();
        let camera = scene.camera.clone();
        let on_resize = Closure::wrap(Box::new(move || {
            canvas.set_width(canvas.client_width() as u32);
            canvas.set_height(canvas.client_height() as u32);
            gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
            camera.on_window_resize(&gl);
        }) as Box<dyn Fn()>);
        window().set_onresize(Some(on_resize.as_ref().unchecked_ref()));
        on_resize.forget();
    }

    {   // render loop callback
        let render_loop = Rc::new(RefCell::new(None));
        let render_loop_2 = render_loop.clone();
        *render_loop_2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            scene.next_frame(&gl);
            scene.draw(&gl);

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(render_loop.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(render_loop_2.borrow().as_ref().unwrap());
    }

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
