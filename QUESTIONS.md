# Questions
## Tại sao họ có thể lấy struct của mình đem đi render được?
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

impl RenderOnce for CounterDisplay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("Likes: {}", self.value))
    }
}

impl RenderOnce for IncrementButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(rgb(BORDER_COLOR))
            .text_color(rgb(BUTTON_FOREGROUND_COLOR))
            .bg(rgb(BUTTON_BACKGROUND_COLOR))
            .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
            .child(self.label)
            .when_some(self.on_click, |this, on_click| {
                return this.on_click(move |eve, win, app| (on_click)(eve, win, app));
            })
    }
}
```

## Trong Pomodoro, mình tư duy: 25 phút mình phải làm xong hết. Nhưng nếu không làm xong được, Làm sao để vẫn giữ tư duy trên để tránh bị mất tập trung?
:::result[Kết quả]
- **Chia nhỏ việc hơn nữa.**
- Ví dụ:
  - “Viết API login”.
      Nhưng thật ra nó gồm:
      - Tạo route
      - Validate input
      - Hash password
      - Query DB
      - Trả response
:::

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

## Tại sao handler lại là function, và tại sao nó chạy dù ko có on_click()?
```rust
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
```
