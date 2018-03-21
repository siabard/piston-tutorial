# Part 2: Moving Square

part1 에서 간단한 예제를 해보았으니, 이제는 키보드로 움직이는 다음단계로 넘어가보도록 합시다. 그 작업전에 코드를 조금 정리합니다.

게임정보를 저장하는 구조체를 구성합니다.

```rust
struct Game {
    rotation: f64;
}
```

다음으로 구현을 합니다. 기존에 있던 게임 및 렌더링 로직을 `on_update`, `on_render`함수로 변경할 겁니다.

```rust
impl Game {
    fn new() -> Game {
        Game { rotation: 0.0 }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
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
                center.rot_rad(self.rotation).trans(-50.0, -50.0),
                g,
            );
        });
    }
}
```

이제 Game 구조체를 생성하고, 마지막으로 해당 구조체를 사용하도록 이벤트 루프를 고칩니다.

```rust
let mut game: Game = Game::new();

while let Some(e) = window.next() {
    if let Some(r) = e.render_args() {
        game.on_render(r, &mut window, &e);
    }

    if let Some(u) = e.update_args() {
        game.on_update(u);
    }
}
```

한가지 주목할 것은 렌더링 이벤트를 `e.render_args()`가 발생하는 시점에서 확인하는 것입니다. 이 시점이 새로운 프레임을 그리는 때입니다. 그리고, 렌더링 부분을 이벤트 루프에서 분리하여, 키보드 이벤트와 렌더링 이벤트가 뒤섞이는 것을 막을 수 있습니다.

`part2.rs` 파일에 해당 내용을 저장하고, Cargo.toml 에 아래를 추가합니다.

```toml
[[bin]]
name="part2"
path="src/bin/part2.rs"
```

마지막으로 `cargo build --bin part2` 를 하면 빌드를 하고, `cargo run --bin part2`를 하면 실행이 됩니다. part1 과 동일한 동작을 합니다.

## 사각형 움직이기

사용자 입력을 얻어오는 것은 다른 이벤트 처리와 유사합니다.

```rust
 if let Some(Button::Keyboard(key)) = e.press_args() {
     game.on_press(key);
 }

if let Some(Button::Keyboard(key)) = e.release_args() {
    game.on_release(key);
}
```

`on_press`, `on_release` 을 작성하기 전에, dt 기반으로한 이동을 하기위해 몇가지 불린 값이 필요합니다. 또한 사각형의 x, y 변수도 추가합니다.

```rust
struct Game {
    rotation: f64,
    x: f64,
    y: f64,
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
}
```

등록된 변수에 대하여 `Game::new()` 부분에서 기본값을 설정하도록 합니다.

```rust
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
```

이제 `on_press`, `on_release` 을 작성합니다.

```rust
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
```

보는 바와 키보드 입력에 대한 누름(`press_args`)과 놓음(`release_args`)에 대한 처리를 하였다. 해당 키에 대하여 `*_d`에 대한 변수 설정도 완료되었습니다.

끝으로 update 부분에서 dt 에 맞추어 이동을 시킵니다.

```rust
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
```

이제 render 에서 x,y 위치에 맞게 사각형을 그려주면 됩니다.

```rust
rectangle(
    red,
    square,
    center
        .trans(self.x, self.y)
        .rot_rad(self.rotation)
        .trans(-50.0, -50.0),
    g,
);
```
