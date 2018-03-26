use piston_window::*;
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use std::f64::consts::PI;
use nalgebra::Vector1;
use nalgebra::Vector2;
use nalgebra::Rotation2;
use nalgebra::Point2;
use nalgebra::{zero, Isometry2};

use ncollide::shape::Cuboid;
use ncollide::query::PointQuery;

pub type Vec1 = Vector1<f64>;
pub type Vec2 = Vector2<f64>;
pub type Rot2 = Rotation2<f64>;
pub type Pnt2 = Point2<f64>;
pub type Cuboid2f = Cuboid<Vec2>;

#[derive(Copy, Clone)]
pub struct Transform {
    pos: Vec2,
    scale: Vec2,
    rot: f64,
}

#[allow(dead_code)]
impl Transform {
    fn new() -> Transform {
        Transform {
            pos: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            rot: 0.0,
        }
    }

    pub fn mov(&mut self, v: Vec2) {
        self.pos = self.pos + v;
    }

    pub fn mov_to(&mut self, v: Vec2) {
        self.pos = v;
    }

    pub fn rot(&mut self, r: f64) {
        self.rot += r;
    }

    pub fn rot_to(&mut self, r: f64) {
        self.rot = r;
    }

    pub fn fwd(&mut self, d: f64) {
        self.pos.x += d * (-self.rot.sin());
        self.pos.y += d * self.rot.cos();
    }
}

pub struct Component {
    trans: Transform,
    sprite: Option<Texture<Resources>>,
}

impl Component {
    fn new() -> Component {
        Component {
            trans: Transform::new(),
            sprite: None,
        }
    }

    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }

    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let t: Transform = self.trans;
        match self.sprite {
            Some(ref sprite) => {
                let (spritex, spritey) = sprite.get_size();
                let (ocx, ocy) = (spritex / 2, spritey / 2);
                image(
                    sprite,
                    view.trans(t.pos.x, t.pos.y)
                        .scale(t.scale.x, t.scale.y)
                        .rot_rad(t.rot)
                        .trans(-(ocx as f64), -(ocy as f64)),
                    g,
                );
            }
            _ => {}
        }
    }
}

pub trait Object {
    fn mov(&mut self, pos: Vec2);
    fn mov_to(&mut self, pos: Vec2);
    fn rot(&mut self, r: f64);
    fn rot_to(&mut self, r: f64);
    fn fwd(&mut self, d: f64);
    fn update(&mut self, dt: f64);
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d);
}

pub struct Tank {
    pub hull: Component,
    pub turret: Component,
    collider: Cuboid2f,
    pub is_destroyed: bool,
    point_to: Vec2,
}

impl Tank {
    pub fn new() -> Tank {
        Tank {
            hull: Component::new(),
            turret: Component::new(),
            point_to: Vec2::new(0.0, 0.0),
            collider: Cuboid2f::new(Vec2::new(38.0, 65.0)),
            is_destroyed: false,
        }
    }

    pub fn point_tur_to(&mut self, x: f64, y: f64) {
        self.point_to = Vec2::new(x, y);
    }

    pub fn calc_tur_pos(&mut self, dt: f64) {
        let mut new_rot = (-(self.point_to.x - self.hull.trans.pos.x))
            .atan2(self.point_to.y - self.hull.trans.pos.y);
        if new_rot == self.turret.trans.rot {
            return;
        }

        if new_rot < self.turret.trans.rot
            && self.turret.trans.rot - new_rot > new_rot + 2.0 * PI - self.turret.trans.rot
        {
            new_rot += 2.0 * PI;
        }

        if new_rot > self.turret.trans.rot
            && new_rot - self.turret.trans.rot > self.turret.trans.rot + 2.0 * PI - new_rot
        {
            new_rot -= 2.0 * PI;
        }

        let rot_speed = 1.0;

        if new_rot > self.turret.trans.rot {
            if new_rot - self.turret.trans.rot > rot_speed * dt {
                self.turret.trans.rot += rot_speed * dt;
            } else {
                self.turret.trans.rot = new_rot;
            }
        } else {
            if self.turret.trans.rot - new_rot > rot_speed * dt {
                self.turret.trans.rot -= rot_speed * dt;
            } else {
                self.turret.trans.rot = new_rot;
            }
        }

        if self.turret.trans.rot > 2.0 * PI {
            self.turret.trans.rot -= 2.0 * PI;
        }

        if self.turret.trans.rot < 0.0 {
            self.turret.trans.rot += 2.0 * PI;
        }
    }

    pub fn collides(&mut self, b: &Bullet) -> bool {
        let bpnt = Pnt2::new(b.bullet.trans.pos.x, b.bullet.trans.pos.y);
        let pos = Isometry2::new(self.hull.trans.pos.clone(), zero());
        self.collider.contains_point(&pos, &bpnt)
    }

    pub fn fire(&self, sprite: Texture<Resources>) -> Bullet {
        let mut bul = Bullet {
            bullet: Component::new(),
            to_be_removed: false,
        };
        bul.mov_to(self.turret.trans.pos);
        bul.rot_to(self.turret.trans.rot);
        bul.fwd(125.0);
        bul.bullet.set_sprite(sprite);
        bul
    }
}

#[allow(dead_code)]
impl Object for Tank {
    fn mov(&mut self, pos: Vec2) {
        self.hull.trans.mov(pos);
    }

    fn mov_to(&mut self, pos: Vec2) {
        self.hull.trans.mov_to(pos);
    }

    fn rot(&mut self, r: f64) {
        self.hull.trans.rot(r);
        self.turret.trans.rot(r);
    }

    fn rot_to(&mut self, r: f64) {
        self.hull.trans.rot_to(r);
        self.turret.trans.rot_to(r);
    }

    fn fwd(&mut self, d: f64) {
        self.hull.trans.fwd(d);
        self.turret.trans.pos = self.hull.trans.pos;
    }

    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        self.hull.render(g, view);
        self.turret.render(g, view);
    }

    fn update(&mut self, dt: f64) {
        self.turret.trans.pos = self.hull.trans.pos;
        self.calc_tur_pos(dt);
    }
}

pub struct Bullet {
    pub bullet: Component,
    pub to_be_removed: bool,
}

impl Object for Bullet {
    fn mov(&mut self, pos: Vec2) {
        self.bullet.trans.mov(pos);
    }

    fn mov_to(&mut self, pos: Vec2) {
        self.bullet.trans.mov_to(pos);
    }

    fn rot(&mut self, r: f64) {
        self.bullet.trans.rot(r);
    }

    fn rot_to(&mut self, r: f64) {
        self.bullet.trans.rot_to(r);
    }

    fn fwd(&mut self, d: f64) {
        self.bullet.trans.fwd(d);
    }

    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        self.bullet.render(g, view);
    }

    fn update(&mut self, dt: f64) {
        let bullet_speed = 200.0;
        self.fwd(bullet_speed * dt);
    }
}
