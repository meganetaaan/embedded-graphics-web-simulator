use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;
use boid::*;
static SCREEN_WIDTH: u16 = 320;
static SCREEN_HEIGHT: u16 = 240;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    let mut display = WebSimulatorDisplay::new((SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32), &output_settings);

    let mut boids: Boids = Boids::new();
    let mut boids_renderer = BoidRenderer::new();
    boids.init();
    boids.update();
    boids_renderer.draw(&mut display, &boids).unwrap();
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        boids.update();
        // clear_screen(&mut display).unwrap();
        boids_renderer.clear(&mut display).unwrap();
        boids_renderer.draw(&mut display, &boids).unwrap();

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
