use gpui::*;

const BACKGROUND_COLOR: u32 = 0x000000;
const FOREGROUND_COLOR: u32 = 0xffffff;
const BORDER_COLOR: u32 = 0x555555;
const BUTTON_BACKGROUND_COLOR: u32 = 0x222222;
const BUTTON_FOREGROUND_COLOR: u32 = 0xFFFFFF;
const BUTTON_HOVER_COLOR: u32 = 0x444444;

struct Counter {
    value: i32,
}

impl Counter {
    fn render_like_button(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(rgb(BORDER_COLOR))
            .text_color(rgb(BUTTON_FOREGROUND_COLOR))
            .bg(rgb(BUTTON_BACKGROUND_COLOR))
            .hover(|style| style.bg(rgba(BUTTON_HOVER_COLOR)))
            .child("Like")
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|data, _, _, cx| {
                    cx.notify();
                    data.value += 1;
                }),
            )
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(BACKGROUND_COLOR))
            .size_full()
            .flex_col()
            .justify_center()
            .items_center()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("{}", &self.value))
            .child(self.render_like_button(cx))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| Counter { value: 0 })
        })
        .unwrap();
    });
}
