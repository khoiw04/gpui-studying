## Cách tạo component
1. Khai báo Props (Types)
```rust
struct MyComponent {
    id: ElementId,
    label: SharedString,
}
```

2. Khai báo Props (Values)
```rust
impl MyComponent {
    fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Self { id: id.into(), label }
    }
}
```

3. Khai báo UI
4. Lắp vào compnents

Gọi Component::new(...) ngay trong hàm render của View cha.

:::result[Kết quả]
`struct` Props -> `impl` Values -> `render` UI -> `impl` Execution
:::

---

## Tại sao handler lại là function, và tại sao nó chạy dù ko có on_click()?
```rust
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
```
:::result[Kết quả]
Do **eventloop của gpui Framework** và **"tin nhắn" của hệ điều hành**:
- HĐH gửi tin
- eventloop nhận
- eventloop render
:::

---

## Cách trait chứa biến "_window: &mut Window, cx: &mut Context<Self>" và đem fn render đi vào khác impl khác
```rust
impl Render for Likes {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(BACKGROUND_COLOR))
            .size_full()
            .justify_center()
            .items_center()
            .gap_2()
            .child(CounterDisplay::new("counter-display", self.likes))
            .child(
                IncrementButton::new("increment-button", "Click".into()).on_click(cx.listener(
                    |data, _, _, app| {
                        data.likes += 1;
                        app.notify();
                    },
                )),
            )
    }
}
```
:::result[Kết quả]
```rust Ví dụ code
<!-- Ownership -->
struct Window { title: String }
struct AppContext { version: String }

<!-- &mut -->
trait MyRender {
    fn render(&mut self, win: &mut Window, cx: &mut AppContext);
}

<!-- &mut -->
struct MyButton {
    label: String,
}

<!-- &mut -->
impl MyRender for MyButton {
    fn render(&mut self, win: &mut Window, cx: &mut AppContext) {}
}


<!-- &mut -->
struct Framework {
    window: Window,
    app_cx: AppContext,
}

<!-- &mut -->
impl Framework {
    fn run_ui(&mut self, component: &mut dyn MyRender) {
        component.render(&mut self.window, &mut self.app_cx);
    }
}
```

Tất cả chỉ là tham chiếu, và không mất quyền ownership
:::

---

## Cách thức nào trong Rust: 1 hàm vừa trao quyền, 1 hàm vừa làm việc mà không mất quyền ownership
```rust Hàm trao quyền
    .when_some(self.on_click, |this, on_click| {
        return this.on_click(move |eve, win, app| (on_click)(eve, win, app));
    })
```

```rust Hàm làm việc
    .child(
        IncrementButton::new("increment-button", "Click".into()).on_click(cx.listener(
            |data, _, _, app| {
                data.likes += 1;
                app.notify();
            },
        )),
    )
```
