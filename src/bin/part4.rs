extern crate piston_tutorial;

extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate piston_window;

use piston_window::*;
use piston_tutorial::object_v2::ObjectV2;

struct Game {
    rotation: f64,
    player: ObjectV2,
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
    scx: f64,
    scy: f64,
}

#[allow(unused_variables)]
impl Game {
    fn new() -> Game {
        Game {
            rotation: 0.0,
            player: ObjectV2::new(),
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
            scx: 300.0,
            scy: 300.0,
        }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
        if self.up_d {
            self.player.fwd(150.0 * upd.dt);
        }

        if self.down_d {
            self.player.fwd(-150.0 * upd.dt);
        }

        if self.left_d {
            self.player.rot(-1.0 * upd.dt);
        }

        if self.right_d {
            self.player.rot(1.0 * upd.dt);
        }
        self.player.update(upd.dt);
    }

    fn on_render(&mut self, ren: RenderArgs, w: &mut PistonWindow, e: &Event) {
        self.scx = (ren.width / 2) as f64;
        self.scy = (ren.height / 2) as f64;

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
        let tank_sprite = assets.join("E-100_Base.png");
        let tank_sprite = Texture::from_path(
            &mut w.factory,
            &tank_sprite,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();

        self.player.set_sprite(tank_sprite);
        let tank_turret = assets.join("E-100_Turret.png");
        let tank_turret = Texture::from_path(
            &mut w.factory,
            &tank_turret,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();
        self.player.set_turret_sprite(tank_turret);
    }

    fn on_mouse_move(&mut self, pos: [f64; 2]) {
        let x = pos[0];
        let y = pos[1];
        self.player.point_tur_to(x - self.scx, y - self.scy);
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

        if let Some(pos) = e.mouse_cursor_args() {
            game.on_mouse_move(pos);
        }
    }
}
