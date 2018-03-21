# Part 3: From a square to a tank

이번에는 실전으로 들어가봅시다. `src/` 디렉토리에 `object.rs` 라는 파일을 생성하는데 해당 파일은 다양한 게임내 객체에 대한 정보를 관리할 것입니다.

한편 해당 파일은 모듈이기때문에 `lib.rs`에 해당하는 정보를 등록합니다.

```rust
extern crate piston_window;

pub mod object;
```

이제 `object.rs`에 `render()` 메서드까지 포함하여 해당 내용을 넣습니다.

```rust
use piston_window::*;

pub struct Object {
    x: f64,
    y: f64,
}

impl Object {
    pub fn new() -> Object {
        Object { x: 0.0, y: 0.0 }
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn render<G>(&self, g: &mut G, view: math::Matrix2d)
    where
        G: Graphics,
    {
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        rectangle(
            red,
            square,
            view.trans(self.x, self.y).trans(-50.0, -50.0),
            g,
        );
    }
}
```

`object` 모듈을 `part3.rs`에 넣습니다.

```rust
use piston_tutorial::object::Object;
```

`Game` 구조체에서 x, y 멤버변수를 빼고, 해당 변수를 `Object` 구조체 player 로 대신합니다. 여기에 x,y 와 관련된 연관 함수 `on_update`, `on_render`도 변경합니다.

```rust
struct Game {
    rotation: f64,
    player: Object,
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
}
```

```rust
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
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform
                .trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            self.player.render(g, center);
        });
    }
```

사각형이 도는 부분은 제거되어있습니다.

이제 탱크를 스라이트를 만듭시다. assets 디렉토리를 생성하고, 이 디렉토리에 원하는 스프라이트 파일(이 예제에서는 `E-100_preview.png`) 를 넣습니다.

이제 새로운 crate 가 필요합니다. `find_foler`는 asset 폴더를 찾아줍니다. 그리고 `gfx` 관련 라이브러리가 필요합니다. `Cargo.toml`에 적용합니다.

```toml
[dependencies]
piston_window="0.77.0"
piston2d-gfx_graphics = "0.50.0"
find_folder = "0.3.0"
gfx = "0.17.1"
gfx_device_gl = "0.15.0"
```

마찬가지로 해당 모듈을 포함해야합니다.

```rust
extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
```

`object.rs`에 아래와 같이 추가합니다.

```rust
pub struct Object {
    x: f64,
    y: f64,
    sprite: Option<Texture<Resources>>,
}
```

```rust
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
```

이제 `render()` 함수에서 sprite 를 출력하도록 바꿉니다.

```rust
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
```

끝으로 `part3.rs`에서 셋업작업을 합니다. `on_load()`함수에서 스프라이트를 읽어들입니다.

```rust
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
```

완성했습니다. 실행하면 탱크를 키로 움직일 수 있습니다.
