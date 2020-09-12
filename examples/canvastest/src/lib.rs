use bakkesmod;
use bakkesmod::prelude::*;

#[plugin_init]
fn on_load() {
    log_console!("This is the canvastest plugin.");

    bakkesmod::register_drawable(Box::new(move |canvas: Canvas| {
        let red = lin_color!(255.0, 0, 0, 255.0);
        canvas.set_color(red);
        canvas.set_position(vec2!(100, 100));
        canvas.draw_string("henlo");

        canvas.set_color_chars(0, 255, 0, 255);
        canvas.set_position(vec2!(500, 200));
        canvas.draw_string_scale("big green text", 5.0, 5.0);

        canvas.set_color(lin_color!(0, 0, 255, 255));
        canvas.set_position(vec2!(300, 300));
        canvas.fill_triangle(vec2!(300, 100), vec2!(400, 200), vec2!(200, 200));
    }));

    log_console!("Drawables have been registered.");
}