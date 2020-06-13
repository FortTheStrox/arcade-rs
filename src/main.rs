extern crate sdl2;
mod events;

use sdl2::pixels::Color;
use events::Events;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.into_canvas()
        .accelerated()
        .build().unwrap();

    // prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }
        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0)); // set brush color to black
        renderer.clear(); // we clear the content of the buffer and fill it w/ prev chosen brush color
        renderer.present(); // swaps the buffers & shows what we have drawn to user
    }


}