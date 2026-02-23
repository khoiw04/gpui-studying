use gpui::{prelude::FluentBuilder, *};

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

#[derive(IntoElement)]
struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Button {
    fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Button {
            id: id.into(),
            label,
            on_click: None,
        }
    }

    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
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
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
            })
            .child(self.label)
    }
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

impl Render for Person {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let person_handle = cx.entity().clone();

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
            .child(Button::new("like-button", "Like".into()).on_click(
                move |_event, _window, cx| {
                    person_handle.update(cx, |person, cx| {
                        person.likes += 1;
                        cx.notify();
                    });
                },
            ))
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
