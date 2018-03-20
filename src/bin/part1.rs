extern crate piston_tutorial;

extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston part 1", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rotation: f64 = 0.0;

    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let center = c.transform.trans(300.0, 300.0);
                let square = rectangle::square(0.0, 0.0, 100.0);
                let red = [1.0, 0.0, 0.0, 1.0];

                rectangle(red, square, center.rot_rad(rotation).trans(-50.0, -50.0), g);
            });
        }

        if let Some(u) = e.update_args() {
            rotation += 3.0 * u.dt;
        }
    }
}
