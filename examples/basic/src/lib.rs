use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::prelude::*;
use embedded_graphics::{
    image::Image,
    image::ImageRawLE,
    pixelcolor::{Rgb565},
    prelude::*,
};
use web_sys::console;

fn draw_image<D>(&raw_image: &ImageRawLE<Rgb565>, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Rgb565>,
{
    use core::convert::TryFrom;
    let (w, h) = display.size().into();
    let (iw, ih) = (raw_image.width(), raw_image.height());
    let (x, y) = (
        i32::try_from(w / 2 - iw / 2).unwrap(),
        i32::try_from(h / 2 - ih / 2).unwrap(),
    );
    let top_left = Point::new(x, y);
    let image = Image::new(&raw_image, top_left);
    image.draw(display)
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut img_display = WebSimulatorDisplay::new((128, 128), &output_settings);

    let output_settings_x2 = OutputSettingsBuilder::new().scale(2).build();
    let mut img_display_x2 = WebSimulatorDisplay::new((128, 128), &output_settings_x2);

    let raw = ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
    draw_image(&raw, &mut img_display).unwrap();
    draw_image(&raw, &mut img_display_x2).unwrap();
    Ok(())
}
