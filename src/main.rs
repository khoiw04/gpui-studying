use gpui::{prelude::FluentBuilder, *};

const BACKGROUND_COLOR: u32 = 0x000000;
const FOREGROUND_COLOR: u32 = 0xffffff;
const BORDER_COLOR: u32 = 0x555555;
const BUTTON_BACKGROUND_COLOR: u32 = 0x222222;
const BUTTON_FOREGROUND_COLOR: u32 = 0xFFFFFF;
const BUTTON_HOVER_COLOR: u32 = 0x444444;

struct CounterContainer {
    value: i16,
}

#[derive(IntoElement)]
struct CounterDisplay {
    id: ElementId,
    value: i16,
}

#[derive(IntoElement)]
struct CounterControls {
    increase_id: ElementId,
    decrease_id: ElementId,
    on_increase_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_decrease_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl CounterDisplay {
    fn new(id: impl Into<ElementId>, value: i16) -> Self {
        Self {
            id: id.into(),
            value,
        }
    }
}

impl CounterControls {
    fn new(increase_id: impl Into<ElementId>, decrease_id: impl Into<ElementId>) -> Self {
        Self {
            increase_id: increase_id.into(),
            decrease_id: decrease_id.into(),
            on_decrease_click: None,
            on_increase_click: None,
        }
    }
    fn on_increase_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_increase_click = Some(Box::new(handler));
        self
    }
    fn on_decrease_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_decrease_click = Some(Box::new(handler));
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

impl RenderOnce for CounterControls {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .child(
                div()
                    .id(self.increase_id)
                    .p_2()
                    .rounded_lg()
                    .border_color(rgb(BORDER_COLOR))
                    .text_color(rgb(BUTTON_FOREGROUND_COLOR))
                    .bg(rgb(BUTTON_BACKGROUND_COLOR))
                    .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
                    .child("+")
                    .when_some(self.on_increase_click, |this, handler| {
                        this.on_click(move |evt, win, app| (handler)(evt, win, app))
                    }),
            )
            .child(
                div()
                    .id(self.decrease_id)
                    .p_2()
                    .rounded_lg()
                    .border_color(rgb(BORDER_COLOR))
                    .text_color(rgb(BUTTON_FOREGROUND_COLOR))
                    .bg(rgb(BUTTON_BACKGROUND_COLOR))
                    .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
                    .child("-")
                    .when_some(self.on_decrease_click, |this, handler| {
                        this.on_click(move |evt, win, app| (handler)(evt, win, app))
                    }),
            )
    }
}
impl Render for CounterContainer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(BACKGROUND_COLOR))
            .size_full()
            .justify_center()
            .items_center()
            .gap_2()
            .child(CounterDisplay::new("counter-display", self.value))
            .child(
                CounterControls::new("counter-increase-button", "couter-decrease-button")
                    .on_increase_click(cx.listener(|this, _, _, app| {
                        this.value += 1;
                        app.notify();
                    }))
                    .on_decrease_click(cx.listener(|this, _, _, app| {
                        this.value -= 1;
                        app.notify();
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| CounterContainer { value: 0 })
        })
        .unwrap();
    });
}
