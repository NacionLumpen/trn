extern crate piston_window;

use piston_window::*;

enum Move {
    Straight,
    TurningLeft,
    TurningRight
}

pub fn rem(lhs: f64, rhs: u32) -> f64 {
    let mut result = lhs;
    let module = rhs as f64;
    while result >= module {
        result -= module;
    }
    while result < 0.0 {
        result += module;
    }
    result
}

#[allow(dead_code)]
fn main() {
    let win_size = [640, 480];
    let speed = 100.0; // px per second
    let angular_speed = 0.05; // radians per second
    
    let window: PistonWindow = WindowSettings::new("TRN", win_size)
        .exit_on_esc(true)
        .into();

    let mut pos = [win_size[0] as f64 / 2.0, win_size[1] as f64 / 2.0];
    let mut dir: f64 = 0.0;
    let mut movement = Move::Straight;
    
    for e in window {
        e.render(|_| {
            e.draw_2d(|c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                rectangle(
                    [1.0; 4],
                    [pos[0], pos[1], 1.0, 1.0],
                    c.transform,
                    g);
            });
        });

        e.update(|args| {
            dir += angular_speed * match movement {
                Move::Straight => 0.0,
                Move::TurningLeft => 1.0,
                Move::TurningRight => -1.0
            };
            
            pos[0] = rem(pos[0] + args.dt * speed * dir.cos(), win_size[0]);
            pos[1] = rem(pos[1] + args.dt * speed * -dir.sin(), win_size[1]);
            println!("{:?}", pos);
        });

        e.press(|button| {
            match button {
                Button::Keyboard(Key::Left) => movement = Move::TurningLeft,
                Button::Keyboard(Key::Right) => movement = Move::TurningRight,
                _ => {}
            }
        });

        e.release(|button| {
            match button {
                Button::Keyboard(Key::Left) | Button::Keyboard(Key::Right) =>
                    movement = Move::Straight,
                _ => {}
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rem_should_not_change_values_in_range() {
        assert_eq!(rem(0.0, 100), 0.0);
        assert_eq!(rem(50.0, 100), 50.0);
    }

    #[test]
    fn rem_should_reduce_values_above_range() {
        assert_eq!(rem(100.0, 100), 0.0);
        assert_eq!(rem(150.0, 100), 50.0);
        assert_eq!(rem(250.0, 100), 50.0);
    }

    #[test]
    fn rem_should_increase_values_below_range() {
        assert_eq!(rem(-100.0, 100), 0.0);
        assert_eq!(rem(-140.0, 100), 60.0);
        assert_eq!(rem(-240.0, 100), 60.0);
    }
}
