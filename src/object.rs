use piston_window::*;
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;

pub struct Object {
    x: f64,
    y: f64,
    sprite: Option<Texture<Resources>>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            x: 0.0,
            y: 0.0,
            sprite: None,
        }
    }

    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];

        match self.sprite {
            None => rectangle(
                red,
                square,
                view.trans(self.x, self.y).trans(-50.0, -50.0),
                g,
            ),
            Some(ref sprite) => {
                image(sprite, view.trans(self.x, self.y).trans(-50.0, -50.0), g);
            }
        }
    }
}
