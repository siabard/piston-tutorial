# Part 5: Gameplay

이번 파트에서는 게임의 기본적인 부분을 구성합니다. - 상대방으로 다른 탱크를 추가하고, 투사체역시 추가됩니다. 해당 투사체로 상대방 탱크를 공격해 파괴합니다. 당분간 코드의 질은 높지않습니다. - 이번 파트에의 핵심은 `nalgebra`와 `ncollide`의 이용입니다.

## Object 구조체의 간소화(Streamlining)

먼저 object struct 을 간소화합시다. 지금까지 해당 객체는 여러모로 모자른 점이 많아습니다. 그 중 하나는 위치와 회전값을 "unwrapped" 해온것입니다. 이부분을 고치기위해 `nalgebra` 라이브러리를 사용합니다.

```toml
## Cargo.toml [dependencies]

nalgebra="*"
```

`lib.rs`에 해당 모듈을 사용하도록 합니다.

```rust
extern crate nalgebra;
```

`object_v3.rs`에 아래와 같은 문구를 추가합니다.

```rust
// object_v3.rs

use nalgebra::Vector2;

pub type Vec2 = Vector2<f64>;
```

이제 해당 구조체에 위치, 회전, 비율을 담당할 구조체를 넣습니다.

```rust
// object_v3.rs

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
```

스프라이트와 렌더링을 담당할 컨테이너를 만듭니다.

```rust
//object_v3.rs

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
```

이제 `object_v3` 구조체를 업데이트합니다.

```rust
// object_v3.rs


pub struct ObjectV3 {
    pub hull: Component,
    pub turret: Component,
    point_to: Vec2,
}

#[allow(dead_code)]
impl ObjectV3 {
    pub fn new() -> ObjectV3 {
        ObjectV3 {
            hull: Component::new(),
            turret: Component::new(),
            point_to: Vec2::new(0.0, 0.0),
        }
    }

    pub fn mov(&mut self, pos: Vec2) {
        self.hull.trans.mov(pos);
    }

    pub fn mov_to(&mut self, pos: Vec2) {
        self.hull.trans.mov_to(pos);
    }

    pub fn rot(&mut self, r: f64) {
        self.hull.trans.rot(r);
        self.turret.trans.rot(r);
    }

    pub fn rot_to(&mut self, r: f64) {
        self.hull.trans.rot_to(r);
        self.turret.trans.rot_to(r);
    }

    pub fn fwd(&mut self, d: f64) {
        self.hull.trans.fwd(d);
        self.turret.trans.pos = self.hull.trans.pos;
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

    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        self.hull.render(g, view);
        self.turret.render(g, view);
    }

    pub fn update(&mut self, dt: f64) {
        self.turret.trans.pos = self.hull.trans.pos;
        self.calc_tur_pos(dt);
    }
}
```

정직하게 이 코드 자체는 좀 지저분합니다. `hull`, `turret`을 public 으로 구성할 수 밖에 없는데 `on_load` 함수에서 스프라이트 지정을 해야하기 때문입니다.

```rust
//part5.rs

self.player.hull.set_sprite(tank_sprite);
self.player.turret.set_sprite(tank_turret);
```

휴.

숨 좀 돌리죠. 아직 안 끝났습니다.

다음 라이브러리는 (현재 튜토리얼의 범위를 넘어 코드를 복잡하게 만들고자하지않는다면 ) - `ncollide` 입니다.

```toml
ncollide="*"
```

```rust
//lib.rs [dependencies]

extern crate ncollide;
```

```rust
// object_v3.rs
use nalgebra::Vector1;
use nalgebra::Vector2;
use nalgebra::Rotation2;
use nalgebra::Point2;

use ncollide::shape::Cuboid;

pub type Vec1 = Vector1<f64>;
pub type Vec2 = Vector2<f64>;
pub type Rot2 = Rotation2<f64>;
pub type Pnt2 = Point2<f64>;
pub type Cuboid2f = Cuboid<Vec2>;
```

이제 실제 게임플레이를 만듭시다. 첫번째로, `Object` trait 을 만듭니다.

```rust
// object_v3.rs

pub trait Object {
    fn mov(&mut self, pos: Vec2);
    fn mov_to(&mut self, pos: Vec2);
    fn rot(&mut self, r: f64);
    fn rot_to(&mut self, r: f64);
    fn fwd(&mut self, d: f64);
    fn update(&mut self, dt: f64);
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d);
}
```

기존의 `ObjectV3`는 `Tank`로 변경했습니다. 위의 `Object`에서 정의된 함수들은 `impl Object for Tank` 내로 옮겨주어야하며, 나머지는 `impl Tank`에 보존합니다. 추가로 충돌 정보에 대해 `Tank` 구조체에 추가하고, 파괴 유무역시 추가합니다.

```rust
pub struct Tank {
    pub hull: Component,
    pub turret: Component,
    collider: Cuboid2f,
    pub is_destroyed: bool,
    point_to: Vec2,
}
```

```rust
// impl tank
    pub fn new() -> Tank {
        Tank {
            hull: Component::new(),
            turret: Component::new(),
            point_to: Vec2::new(0.0, 0.0),
            collider: Cuboid2f::new(Vec2::new(38.0, 65.0)),
            is_destroyed: false,
        }
    }
```

다음으로 새로운 구조체인 `Bullet`을 추가합니다.

```rust
// object_v3.rs

pub sruct Bullet {
    pub bullet: Component,
    pub to_be_removed: bool
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
        let bullet_speed = 200.8;
        self.fwd(bullet_speed * dt);
    }
}
```

`Tank`에 포탄과의 충돌 로직을 추가합니다.

```rust
// impl Tank
    pub fn collides(&mut self, b: &Bullet) -> bool {
        let bpnt = Pnt2::new(b.bullet.trans.pos.x, b.bullet.trans.pos.y);
        let pos = Isometry2::new(self.hull.trans.pos.clone(), zero());
        self.collider.contains_point(&pos, &bpnt)
    }
```

추가로 포탄을 쏘는 로직을 만듭니다.

```rust
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
```

이제 메인로직으로 갑시다.

```rust
// object_v3.rs
use piston_tutorial::object_v3::Bullet;
```

이번에는 또다른 플레이어(player2)를 추가합니다. 해당 플레이어는 컨트롤이 없으며, player1 오른편에 둡니다.
또한 bullet 을 보관할 벡터도 생성합니다.

```rust
// object_v3.rs
// struct Game
    player1: Tank,
    player2: Tank,
    bullets: Vec<Bullet>,
```

```rust
// object_v3.rs
// impl Game
    fn new() -> Game {
        Game {
            rotation: 0.0,
            player1: Tank::new(),
            player2: Tank::new(),
            bullets: Vec::new(),
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
            scx: 300.0,
            scy: 300.0,
        }
    }
```

물론 player2 도 동일한 스프라이트로 정의합니다. 추가로 player2 의 위치를 오른편에 둡니다.

```rust
//part5.rs
//impl Game on_load()

        self.player1.hull.set_sprite(tank_sprite.clone());
        self.player1.turret.set_sprite(tank_turret.clone());

        self.player2.hull.set_sprite(tank_sprite);
        self.player2.turret.set_sprite(tank_turret);

        self.player2.mov_to(Vec2::new(300.0, 0.0));
```

파괴된 탱크의 몸체와 포탑을 구성합니다. Game 오브젝트에 해당하는 리소스를 불러온 후 필요할 때마다 clone 하여 사용하도록 합니다.

```rust
// part5.rs
// struct Game
    hull_destroyed: Option<Texture<Resources>>,
    turret_destroyed: Option<Texture<Resources>>,
    bullet: Option<Texture<Resources>>,
```

`on_update()`에서 총탄이 탱크에 부딪힐 때에 탱크를 파괴하는 것은 물론, 총탄도 화면에서 없애도록 합니다.

```rust
        for bul in &mut self.bullets {
            if self.player1.collides(&bul) {
                self.player1.is_destroyed = true;
                self.player1
                    .hull
                    .set_sprite(self.hull_destroyed.clone().unwrap());
                self.player1
                    .turret
                    .set_sprite(self.hull_destroyed.clone().unwrap());
                bul.to_be_removed = true;
            }

            if self.player2.collides(&bul) {
                self.player2.is_destroyed = true;
                self.player2
                    .hull
                    .set_sprite(self.hull_destroyed.clone().unwrap());
                self.player2
                    .turret
                    .set_sprite(self.hull_destroyed.clone().unwrap());
                bul.to_be_removed = true;
            }
            bul.update(upd.dt);
        }

        self.bullets.retain(|ref bul| bul.to_be_removed == false);
```

이제 발사!

```rust
// part5.rs
// main() event loop
        if let Some(button) = e.release_args() {
            game.on_release(button);
        }
```

```rust
//part5.rs
// impl Game
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
///
```

이제 렌더링 부분을 추가합니다.

```rust
// part5.rs
// impl Game
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
```
