// src/main.rs

extern crate sdl2;

// #[macro_use] asks the compiler to import the macros defined in the `events`
// module. This is necessary because macros cannot be namespaced -- macro
// expansion happens before the concept of namespace even starts to _exist_ in
// the compilation timeline.
#[macro_use]
mod events;

use sdl2::pixels::Color;

struct_events!{
    // These are the current keyboard buttons that programmed in
    // if you don't see a key in here then by default it does nothing (so far)
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_q: Q
    },
    else: {
        quit: Quit {..}
    }
}

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

        if (events.now.key_escape == Some(true)) || (events.now.key_q == Some(true)) {
            break;
        }
        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0)); // set brush color to black
        renderer.clear(); // we clear the content of the buffer and fill it w/ prev chosen brush color
        renderer.present(); // swaps the buffers & shows what we have drawn to user
    }


}