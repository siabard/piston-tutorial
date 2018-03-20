# Part 1 : Hello, Piston!

기본적으로 rust 와 cargo 를 설치했어야합니다. rust 와 cargo 의 설치는 공식 홈페이지를 참조해주세요.

<!--- 
(part0 가 쓰여지면 해당 부분을 추가하겠음)
-->

## 프로젝트만들기

터미널상에서 아래와 같이 입력합니다.

```
cargo new --bin piston-tutorial
```

<!---  
Rust에서 dash(-)와 underscore(_)에 대한 논쟁인 지속적으로 계속되고 있다. crate의 경우에는 두 기호를 명확하게 구분하고 있지못하며, rustfmt 및 cargo check 등에서는 underscore를 강제하는등 공식 툴에서도 규칙이 여러가지이다.

조심해서 스스로 규칙을 강제하는 것 이외에는 뾰족한 방법이 없다.

역자는 clojrue 등에서 제시하는 물리적 파일 구조에는 dash를, 내부 명칭에는 underscore를 이용하는 방식을 쓴다.
-->

작업디렉토리가 만들어지고 기본적인 cargo 환경설정이 등록됩니다.

작업디렉토리안에는 `Cargo.toml` 과 `src` 디렉토리가 만들어집니다. 이 안에는 `main.rs` 파일이 있으며 기본적인 'Hello World' 프로그램이 이미 작성되어 있습니다.

다만 표준적이라고하기는 그렇지만, 많은 개발자들이 이용하는 방식에 따라 작업디렉토리에는 `lib.rs` 를 생성하고, `main.rs` 는 `src/bin` 에 넣는 방법을 사용하도록 합니다.

또한 각 장의 파일은 별도의 파일로 구분하여 `part1.rs`, `part2.rs` 식으로 생성합니다.

`Cargo.toml` 은 프로젝트의 설정파일입니다. 파일을 열면 아래와 유사하게 되어있습니다.

```
[package]
name = "piston-tutorial"
version = "0.1.0"
authors = ["StelarCF <StelarCF@gmail.com>"]
```

authors 같은 경우에는 git 의 user.name 및 user.email 설정정보를 가져옮니다.

아래에 다음과 같이 의존성 정보를 넣습니다.

```
[dependencies]
piston_window="0.77.0"
```

이제 `main.rs` 파일대신에 `src/bin` 에 `part1.rs` 파일을 쓰도록 합시다.

`Cargo.toml` 에 아래와 같이 추가합니다.

```
[[bin]]
name="part1"
path="src/bin/part1.rs"
```

이제 앞으로 `cargo` 명령 실행시 `--bin` 옵션을 넣고 해당 name 을 지정하면 해당하는 path 의 파일을 이용해서 작업이 진행됩니다.

```
cargo check --bin part1
cargo build --bin part1
cargo run --bin part1
```

이제 `part1.rs` 파일을 열고 아래와 같이 입력합니다.

```
extern crate piston_tutorial;

extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston part 1", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {}
}
```

이후에 `cargo build --bin part1` 명령을 내리면 빌드가 됩니다. `warning: unused variable: 'e'`와 같은 경고가 나오지만 여기서는 무시합니다. 이제 프로그램을 `cargo run --bin part1`이나 `target/` 에 있는 실행파일을 실행시키면 됩니다.

이제 첫번째 윈도가 완성되었습니다.

## 어떤 일이 일어났는가?

대부분의 코드는 매우 직관적입니다. `piston_window` crate 를 포함시켰고, 전역 네임스페이스에 해당 내용을 적재합니다. 이후에 "piston part 1" 이라는 창제목으로 주어진 크기의 창을 생성하되, esc 키가 눌리면 종료됩니다.

`while let Some(e) = window.next()` 가 조금 이상하게 보입니다. 이벤트 루프인 것은 알지만 `window.next()`의 결과가 iterator 입니다.

실제로는 매우 단순합니다. windows 에 반복작업을 진행하면서 이벤트(`e.event`)를 받습니다.

## 사각형 출력

이제 사각형을 출력해봅시다. 이벤트 루프에 `PistonWindow`의 `draw_2d`를 호출합니다. 해당 함수는 이벤트개체와 두 개의 인자 - 그래픽 컨텍스트와 그래픽 인스턴스 - 를 받는 람다함수를 인자로 받습니다.

```
window.draw_2d(&e, |c, g|{
```

우선 화면을 검은색으로 지웁니다.

```
clear([0.0, 0.0, 0.0, 1.0], g);
```

다음으로 화면의 가운데에 대한 변환값과 사각형의 크기와 색상을 지정합니다.

```
let center = c.transform.trans(300.0, 300.0); // (300, 300) 위치로 보내라
let square = rectangle::square(0.0, 0.0, 100.0); // (0, 0 ) 위치에 가로, 세로 100 pixel 사각형
let red = [1.0, 0.0, 0.0, 1.0];
```

끝으로 사각형을 노출합니다.

```
rectangle(red, square, center.trans(-50.0, -50.0), g); // 사각형의 절반만큼 좌상단으로 이동 (화면 가운데 맞춤)
```

마지막으로 회전을 시킵니다. `piston_window.event`에서 `update_args()` 를 통해 deltaT (시간에 대한 경과값)을 얻어 update event 에서 처리합니다.

먼저 이벤트 루프 바깥쪽에 회전값을 정의합니다. 이 값은 변경 가능해야하므로 `mut`를 붙입니다.

```
let mut rotation : f64 = 0.0; // 회전 값
```

다음 이벤트루프내에서 `update_args()`가 생성되는 이벤트가 발생할 때마다 회전각을 변경해야합니다.

```
if let Some(u) = e.update_args() {
    rotation += 3.0 * u.dt;
}
```

마지막으로 기존의 사각형 노출부분은 `piston_window.event`에서 `render_args()`가 발생하는 이벤트에서만으로도 충분합니다. 해당 노출시 회전각만큼 돌립니다.

```
if let Some(r) = e.render_args() {
    window.draw_2d(&e, |c, g| {
        clear([0.0, 0.0, 0.0, 1.0], g);
        let center = c.transform.trans(300.0, 300.0);
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];

        rectangle(red, square, center.rot_rad(rotation).trans(-50.0, -50.0), g);
    });
}
```

전체소스는 아래와 같습니다.

```
extern crate piston_tutorial;

extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston part 1", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rotation: f64 = 0.0;

    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let center = c.transform.trans(300.0, 300.0);
                let square = rectangle::square(0.0, 0.0, 100.0);
                let red = [1.0, 0.0, 0.0, 1.0];

                rectangle(red, square, center.rot_rad(rotation).trans(-50.0, -50.0), g);
            });
        }

        if let Some(u) = e.update_args() {
            rotation += 3.0 * u.dt;
        }
    }
}
```
