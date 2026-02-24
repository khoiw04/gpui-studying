// Thứ tự viết components: https://gemini.google.com/share/c8287f0d4427
use gpui::{prelude::FluentBuilder, *};

const BACKGROUND_COLOR: u32 = 0x000000;
const FOREGROUND_COLOR: u32 = 0xffffff;
const BORDER_COLOR: u32 = 0x555555;
const BUTTON_BACKGROUND_COLOR: u32 = 0x222222;
const BUTTON_FOREGROUND_COLOR: u32 = 0xFFFFFF;
const BUTTON_HOVER_COLOR: u32 = 0x444444;

struct Likes {
    likes: i16,
}

#[derive(IntoElement)]
struct CounterDisplay {
    id: ElementId,
    value: i16,
}

#[derive(IntoElement)]
struct IncrementButton {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl CounterDisplay {
    fn new(id: impl Into<ElementId>, value: i16) -> Self {
        Self {
            id: id.into(),
            value,
        }
    }
}

impl IncrementButton {
    fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Self {
            id: id.into(),
            label,
            on_click: None,
        }
    }
    fn on_click(mut self, func: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(func));
        self
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
                return this.on_click(move |evt, wind, app| (on_click)(evt, wind, app));
            })
    }
}

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

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| Likes { likes: 0 })
        })
        .unwrap();
    })
}
