extern crate piston_tutorial;

extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate piston_window;

use piston_window::*;
use piston_tutorial::object::Object;

struct Game {
    rotation: f64,
    player: Object,
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
}

#[allow(unused_variables)]
impl Game {
    fn new() -> Game {
        Game {
            rotation: 0.0,
            player: Object::new(),
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
        }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
        if self.up_d {
            self.player.mov(0.0, -150.0 * upd.dt);
        }

        if self.down_d {
            self.player.mov(0.0, 150.0 * upd.dt);
        }

        if self.left_d {
            self.player.mov(-150.0 * upd.dt, 0.0);
        }

        if self.right_d {
            self.player.mov(150.0 * upd.dt, 0.0);
        }
    }

    fn on_render(&self, ren: RenderArgs, w: &mut PistonWindow, e: &Event) {
        w.draw_2d(e, |c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            let center = c.transform
                .trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            self.player.render(g, center);
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

    fn on_load(&mut self, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let tank_sprite = assets.join("E-100_preview.png");
        let tank_sprite = Texture::from_path(
            &mut w.factory,
            &tank_sprite,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();
        self.player.set_sprite(tank_sprite);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston part 2", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game: Game = Game::new();
    game.on_load(&mut window);

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
