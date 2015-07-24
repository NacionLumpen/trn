extern crate piston_window;

use piston_window::*;

#[allow(dead_code)]
fn main() {
    let window: PistonWindow = WindowSettings::new("TRN", [640, 480])
        .exit_on_esc(true)
        .into();
    for _ in window {}
}
