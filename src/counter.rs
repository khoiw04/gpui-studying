use gpui::*;

const BACKGROUND_COLOR: u32 = 0x000000;
const FOREGROUND_COLOR: u32 = 0xffffff;
const BORDER_COLOR: u32 = 0x555555;
const BUTTON_BACKGROUND_COLOR: u32 = 0x222222;
const BUTTON_FOREGROUND_COLOR: u32 = 0xFFFFFF;
const BUTTON_HOVER_COLOR: u32 = 0x444444;

struct Person {
    first_name: SharedString,
    last_name: SharedString,
    likes: u16,
}

impl Person {
    fn render_name(&self) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(BACKGROUND_COLOR))
            .justify_center()
            .items_center()
            .text_2xl()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("{} {}", &self.first_name, &self.last_name))
    }
}

impl Person {
    fn render_likes(&self) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("Likes: {}", self.likes))
    }
}

impl Person {
    fn handle_increment(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.likes += 1;
        cx.notify();
    }

    fn render_like_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let style = |banana: StyleRefinement| banana.bg(rgb(BUTTON_HOVER_COLOR));
        div()
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(rgb(BORDER_COLOR))
            .text_color(rgb(BUTTON_FOREGROUND_COLOR))
            .bg(rgb(BUTTON_BACKGROUND_COLOR))
            .hover(style)
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_increment))
            .child("Like")
    }
}

impl Render for Person {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(BACKGROUND_COLOR))
            .size_full()
            .justify_center()
            .items_center()
            .gap_2()
            .child(self.render_name())
            .child(self.render_likes())
            .child(self.render_like_button(cx))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| Person {
                first_name: "Mick".into(),
                last_name: "Jagger".into(),
                likes: 0,
            })
        })
        .unwrap();
    })
}
