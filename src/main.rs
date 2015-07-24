extern crate piston_window;

use piston_window::*;

#[allow(dead_code)]
fn main() {
    let win_size = [640, 480];
    
    let window: PistonWindow = WindowSettings::new("TRN", win_size)
        .exit_on_esc(true)
        .into();

    let mut pos = [win_size[0] as f64 / 2.0, win_size[1] as f64 / 2.0];
    
    for frame in window {
        frame.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            rectangle(
                [1.0; 4],
                [pos[0], pos[1], 1.0, 1.0],
                c.transform,
                g);
        });

        pos[1] += 1.0;
        if pos[1] as u32 > win_size[1] {
            pos[1] -= win_size[1] as f64;
        }
    }
}
