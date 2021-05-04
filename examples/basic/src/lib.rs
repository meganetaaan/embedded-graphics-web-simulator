use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::prelude::*;
use embedded_graphics::{
 image::*,
 pixelcolor::Rgb565,
 prelude::*,
};
use wio_splash::WioSplash;
// This is like the main function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
 // This provides better error messages in debug mode.
 // It's disabled in release mode so it doesn't bloat up the file size.
 #[cfg(debug_assertions)]
 console_error_panic_hook::set_once();
 let output_settings = OutputSettingsBuilder::new().scale(2).build();
 let mut display = WebSimulatorDisplay::new((320, 240), &output_settings);
 let raw = ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86,
64);
 let splash = WioSplash::new(Rgb565::GREEN, raw);
 splash.draw(&mut display).unwrap();
 Ok(())
}
