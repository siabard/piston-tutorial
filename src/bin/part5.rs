extern crate piston_tutorial;

extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate piston_window;

use piston_window::*;
use piston_tutorial::object_v3::Tank;
use piston_tutorial::object_v3::Object;
use piston_tutorial::object_v3::Bullet;
use piston_tutorial::object_v3::Vec2;
use gfx_device_gl::Resources;

struct Game {
    rotation: f64,
    player1: Tank,
    player2: Tank,
    hull_destroyed: Option<Texture<Resources>>,
    turret_destroyed: Option<Texture<Resources>>,
    bullet: Option<Texture<Resources>>,
    bullets: Vec<Bullet>,
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
            player1: Tank::new(),
            player2: Tank::new(),
            bullets: Vec::<Bullet>::new(),
            hull_destroyed: None,
            turret_destroyed: None,
            bullet: None,
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
            self.player1.fwd(150.0 * upd.dt);
        }

        if self.down_d {
            self.player1.fwd(-150.0 * upd.dt);
        }

        if self.left_d {
            self.player1.rot(-1.0 * upd.dt);
        }

        if self.right_d {
            self.player1.rot(1.0 * upd.dt);
        }
        self.player1.update(upd.dt);
        self.player2.update(upd.dt);

        for bul in &mut self.bullets {
            if self.player1.collides(&bul) {
                self.player1.is_destroyed = true;
                self.player1
                    .hull
                    .set_sprite(self.hull_destroyed.clone().unwrap());
                self.player1
                    .turret
                    .set_sprite(self.turret_destroyed.clone().unwrap());
                bul.to_be_removed = true;
            }

            if self.player2.collides(&bul) {
                self.player2.is_destroyed = true;
                self.player2
                    .hull
                    .set_sprite(self.hull_destroyed.clone().unwrap());
                self.player2
                    .turret
                    .set_sprite(self.turret_destroyed.clone().unwrap());
                bul.to_be_removed = true;
            }
            bul.update(upd.dt);
        }

        self.bullets.retain(|ref bul| bul.to_be_removed == false);
    }

    fn on_render(&mut self, ren: RenderArgs, w: &mut PistonWindow, e: &Event) {
        self.scx = (ren.width / 2) as f64;
        self.scy = (ren.height / 2) as f64;

        w.draw_2d(e, |c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            let center = c.transform
                .trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            self.player1.render(g, center);
            self.player2.render(g, center);

            for bul in &self.bullets {
                bul.render(g, center);
            }
        });
    }

    fn on_press(&mut self, key: Button) {
        match key {
            Button::Keyboard(key) => match key {
                Key::Up => self.up_d = true,
                Key::Down => self.down_d = true,
                Key::Left => self.left_d = true,
                Key::Right => self.right_d = true,

                _ => {}
            },
            _ => {}
        }
    }

    fn on_release(&mut self, key: Button) {
        match key {
            Button::Keyboard(key) => match key {
                Key::Up => self.up_d = false,
                Key::Down => self.down_d = false,
                Key::Left => self.left_d = false,
                Key::Right => self.right_d = false,

                _ => {}
            },
            Button::Mouse(button) => match button {
                MouseButton::Left => {
                    self.bullets
                        .push(self.player1.fire(self.bullet.clone().unwrap()));
                }
                _ => {}
            },
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

        let tank_turret = assets.join("E-100_Turret.png");
        let tank_turret = Texture::from_path(
            &mut w.factory,
            &tank_turret,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();

        let tank_dest_hull = assets.join("E-100_Base_destroyed.png");
        let tank_dest_hull = Texture::from_path(
            &mut w.factory,
            &tank_dest_hull,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();

        let tank_dest_turret = assets.join("E-100_Turret_destroyed.png");
        let tank_dest_turret = Texture::from_path(
            &mut w.factory,
            &tank_dest_turret,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();

        let bullet_sprite = assets.join("Bullet.png");
        let bullet_sprite = Texture::from_path(
            &mut w.factory,
            &bullet_sprite,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();

        self.hull_destroyed = Some(tank_dest_hull);
        self.turret_destroyed = Some(tank_dest_turret);
        self.bullet = Some(bullet_sprite);

        self.player1.hull.set_sprite(tank_sprite.clone());
        self.player1.turret.set_sprite(tank_turret.clone());

        self.player2.hull.set_sprite(tank_sprite);
        self.player2.turret.set_sprite(tank_turret);

        self.player2.mov_to(Vec2::new(300.0, 0.0));
    }

    fn on_mouse_move(&mut self, pos: [f64; 2]) {
        let x = pos[0];
        let y = pos[1];
        self.player1.point_tur_to(x - self.scx, y - self.scy);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston part 5", [600, 600])
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

        if let Some(button) = e.press_args() {
            game.on_press(button);
        }

        if let Some(button) = e.release_args() {
            game.on_release(button);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            game.on_mouse_move(pos);
        }
    }
}
