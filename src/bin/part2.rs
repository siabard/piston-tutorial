extern crate piston_tutorial;

extern crate piston_window;

use piston_window::*;

struct Game {
    rotation: f64,
    x: f64,
    y: f64,
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            rotation: 0.0,
            x: 0.0,
            y: 0.0,
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
        }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
        if self.up_d {
            self.y += (-50.0) * upd.dt;
        }

        if self.down_d {
            self.y += (50.0) * upd.dt;
        }

        if self.left_d {
            self.x += (-50.0) * upd.dt;
        }

        if self.right_d {
            self.x += (50.0) * upd.dt;
        }
    }

    fn on_render(&self, ren: RenderArgs, w: &mut PistonWindow, e: &Event) {
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans(300.0, 300.0);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];

            rectangle(
                red,
                square,
                center
                    .trans(self.x, self.y)
                    .rot_rad(self.rotation)
                    .trans(-50.0, -50.0),
                g,
            );
        });
    }

    fn on_press(&mut self, key: Key) {
        match key {
            Key::Up => self.up_d = true,
            Key::Down => self.down_d = true,
            Key::Left => self.left_d = true,
            Key::Right => self.right_d = true,

            _ => {}
        }
    }

    fn on_release(&mut self, key: Key) {
        match key {
            Key::Up => self.up_d = false,
            Key::Down => self.down_d = false,
            Key::Left => self.left_d = false,
            Key::Right => self.right_d = false,

            _ => {}
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston part 2", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game: Game = Game::new();

    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            game.on_render(r, &mut window, &e);
        }

        if let Some(u) = e.update_args() {
            game.on_update(u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.on_press(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            game.on_release(key);
        }
    }
}
