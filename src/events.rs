extern crate sdl2;

macro_rules! struct_events {
    ( 
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },
        
        // Match against a pattern
        else: { $( $e_alias:ident : $e_sdl:pat ),* }
    ) => {
        use sdl2::EventPump;
        pub struct ImmediateEvents {
            // For every keyboard event, we have an Option<bool>
            // Some(true)  => Was just pressed
            // Some(false) => Was just released
            // None        => Nothing happening _now_
            $( pub $k_alias: Option<bool>, )*
            $( pub $e_alias: bool),*
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    // When reinitialized, nothing has yet happened, so all are
                    // set to None
                    $( $k_alias: None, )*
                    $( $e_alias: false),*
                }
            }
        }
        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            // true => pressed
            // false => not pressed
            $( pub $k_alias: bool),*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    // By default, initialize every key with _not pressed_
                    $( $k_alias: false),*
                }
            }

            // update the events record
            pub fn pump(&mut self) {
                self.now = ImmediateEvents::new();

                // If the SDL context is dropped, the poll_iter() wills imply stop
                // yielding any input.
                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        KeyDown { keycode, .. } => match keycode {
                            // $( ... ),* containing $k_sdl and $k_alias means:
                            //     "for every element ($k_alias : $k_sdl) pair,
                            //      check whether the keycode is Some($k_sdl). If
                            //      it is, then set the $k_alias fields to true."
                            $(
                                Some($k_sdl) => {
                                    // Prevent multiple presses when keeping a key down
                                    // was previous not pressed?
                                    if !self.$k_alias {
                                        self.now.$k_alias = Some(true);
                                    }

                                    self.$k_alias = true;
                                }
                            ),* // and add a comma after every option
                            _ => {}
                        },
                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Key released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),*
                            _ => {}
                        },
                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                        )*,
                        _ => {}
                    }
                }
            }
        }
    }
    
}
