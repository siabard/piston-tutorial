# Part 4: The Mouse ( and various improvements )

이번에는 아래와 같은 일을 합니다.

* 움직임을 정상적으로 보이게 하기
* 탱크를 여러개의 스프라이트로 구성하기 (포탑과 차체를 분리하여 회전하도록)
* 마우스로 움직이게 하기

`part4.rs`를 `src/bin`에 만들고 `Cargo.toml`에 추가합니다.

```toml
[[bin]]
name="part4"
path="src/bin/part4.rs"
```

## 이동과 포탑

현재 회전 값을 object 에 넣습니다. 여기서는 `object_v2.rs` 만들어 part3 과는 다르게 짜보겠습니다.

`src`에 `object_v2.rs`를 만들고 `lib.rs`에 추가합니다.

```rust
pub mod object_v2;
```

```rust
rot: f64
```

```rust
pub fn rot(&mut self, r: f64) {
    self.rot += r;
}

pub fn rot_to(&mut self, r: f64) {
    self.rot = r;
}
```

현재 지정된 곳으로 이동하는 곳으로 이동하는 함수를 추가합니다.

```rust
pub fn fwd(&mut self, d: f64) {
    self.x += d * (-self.rot.sin());
    self.y += d * self.rot.cos();
}
```

마지막으로 스프라이트의 센터를 지정하여, 회전을 표현합니다.

```rust
Some(ref sprite) => {
    let (spritex, spritey) = sprite.get_size();
    let (ocx, ocy) = (spritex / 2, spritey / 2);
    image(
        sprite,
        view.trans(self.x, self.y)
            .rot_rad(self.rot)
            .trans(-(ocx as f64), -(ocy as f64)),
        g,
    );
}
```

이제 포탑을 올려봅시다. 필요한 속성을 추가합니다.

```rust
tur_x: f64,
tur_y: f64,
rot_tur: f64,
turret: Option<Texture<Resources>>,
```

포탑 스프라이트에 대한 처리와, 포탑을 단위시간마다 일정하게 회전하게하는 함수를 추가합니다.

```rust
pub fn set_turret_sprite(&mut self, sprite: Texture<Resources>) {
    self.turret = Some(sprite);
}

pub fn point_tur_to(&mut self, x: f64, y: f64) {
    self.tur_x = x;
    self.tur_y = y;
}

pub fn update(&mut self, dt: f64) {
    self.calc_tur_pos(dt);
}

pub fn calc_tur_pos(&mut self, dt: f64) {
    let mut new_rot = (-(self.tur_x - self.x)).atan2(self.tur_y - self.y);
    if new_rot == self.rot_tur {
        return;
    }

    if new_rot < self.rot_tur && self.rot_tur - new_rot > new_rot + 2.0 * PI - self.rot_tur {
        new_rot += 2.0 * PI;
    }

    if new_rot > self.rot_tur && new_rot - self.rot_tur > self.rot_tur + 2.0 * PI - new_rot {
        new_rot -= 2.0 * PI;
    }

    let rot_speed = 1.0;

    if new_rot > self.rot_tur {
        if new_rot - self.rot_tur > rot_speed * dt {
            self.rot_tur += rot_speed * dt;
        } else {
            self.rot_tur = new_rot;
        }
    } else {
        if self.rot_tur - new_rot > rot_speed * dt {
            self.rot_tur -= rot_speed * dt;
        } else {
            self.rot_tur = new_rot;
        }
    }

    if self.rot_tur > 2.0 * PI {
        self.rot_tur -= 2.0 * PI;
    }

    if self.rot_tur < 0.0 {
        self.rot_tur += 2.0 * PI;
    }
}
```

`render()`함수에도 추가합니다.

```rust
match self.turret {
    None => {}
    Some(ref sprite) => {
        let (spritex, spritey) = sprite.get_size();
        let (ocx, ocy) = (spritex / 2, spritey / 2);
        image(
            sprite,
            view.trans(self.x, self.y)
                .rot_rad(self.rot_tur)
                .trans(-(ocx as f64), -(ocy as f64)),
            g,
        );
    }
}
```

이제 포탑과 차체를 따로따로 스프라이트로 만듭니다. `E-100_Base.png`와 `E-100_Turret.png` 두 파일을 사용합니다.
`part4.rs`의 `on_load()` 함수는 아래와 같이 바뀝니다.

```rust
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
```

## 마우스 이동

PI 값을 쓰기 위해 아래줄을 `object_v2.rs`에 추가합니다.

```rust
use std::f64::consts::PI;
```

이제부터 화면의 가운데에 머물게 합니다. `part4.rs`에서 `Game`에 스크린 위치를 추가합니다.

```rust
scx: f64,
scy: f64,
```

```rust
pub on_render(&mut self, ren: RenderArgs, e: PistonWindow {
    self.scx = (ren.width / 2) as f64;
    self.scy = (ren.height / 2) as f64;
    e.draw_2d(|c, g| {
        clear([0.8, 0.8, 0.8, 1.0], g);
        let center = c.transform.trans(self.scx, self.scy);
    });
}
```

이벤트 루프에서 마우스 이동에 대한 부분을 가져옵니다.

```rust
if let Some(pos) = e.mouse_cursor_args() {
    game.on_mouse_move(pos);
}
```

`Game`에 `on_mouse_move()`함수를 추가합니다.

```rust
fn on_mouse_move(&mut self, pos: [f64; 2]) {
    let x = pos[0];
    let y = pos[1];
    self.player.point_tur_to(x - self.scx, y - self.scy);
}
```

키보드로 가로세로로 움직였던 탱크를 이제 전진, 후진, 회전하도록 `on_update()` 을 수정합니다.

```rust
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
```

`object_v2.rs`에서 스프라이트를 출력할 때 탱크의 크기를 조금 작게 만듭니다.

```rust
image(
    sprite,
    view.trans(self.x, self.y)
        .scale(0.75, 0.75)
        .rot_rad(self.rot)
        .trans(-(ocx as f64), -(ocy as f64)),
    g,
);
```

```rust
image(
    sprite,
    view.trans(self.x, self.y)
        .scale(0.75, 0.75)
        .rot_rad(self.rot_tur)
        .trans(-(ocx as f64), -(ocy as f64)),
    g,
);
```

완성되었습니다.
