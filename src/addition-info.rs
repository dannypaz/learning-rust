use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;

// Result ... value is empty (empty tuple) and error is string
pub fn main() -> Result<(), String> {
    // unwrap will either Panic on error, or return the value
    // init() returns a "Result"
    let sdl_context = sdl2::init().unwrap();

    // if there is an error, then we will just send back our own error. We are not
    // able to get the error object the "expect" method
    let sdl_context = sdl2::init().expect("What the heck");

    // We have the Result which is usually (Value, Error Value)
    let sdl_context = sdl2::init();

    // We can match and then do something on the Err
    let context = match sdl_context {
        Ok(result) => result,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    // The new new with shorthand is using the question mark
    let video_subsystem = sdl_context.video()?;

    // This will be returned. If you add a semi-colon then nothing will be returned.
    // last statement will be returned if no semi-colon
    video_subsystem
}
