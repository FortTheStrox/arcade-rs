extern crate sdl2;
use sdl2::EventPump;

pub struct Events {
    pump: EventPump,

    pub quit: bool,
    pub key_escape: bool,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
        }
    }

    // update the events record
    pub fn pump(&mut self) {
        // If the SDL context is dropped, the poll_iter() wills imply stop
        // yielding any input.
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                // We do not care about the values of its attributes, and so we simply ignore them using ..
                Quit {..} => self.quit = true,

                KeyDown { keycode, ..} => 
                    match keycode {
                        Some(Escape) => self.key_escape = true,
                        _ => {}
                }

                KeyUp { keycode, ..} =>
                    match keycode {
                        Some(Escape) => self.key_escape = false,
                        _ => {}
                }

                _ => {}
            }
        }
    }
}