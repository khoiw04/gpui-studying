use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x333333))
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(format!("{}", &self.text))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld {
                text: "Hello gpui!".into(),
            })
        })
        .unwrap();
    })
}
