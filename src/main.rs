use gpui::{
    App, Application, Bounds, Context, KeyBinding, Menu, MenuItem, TitlebarOptions, Window,
    WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size,
};

actions!(tilde, [Quit]);

#[derive(Default)]
struct Tilde {
    count: u64,
}

impl Render for Tilde {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0x18181b))
            .text_color(rgb(0xf4f4f5))
            .child(div().text_3xl().child("Hello, GPUI!"))
            .child(
                div()
                    .text_color(rgb(0xa1a1aa))
                    .child("Your Rust desktop app starts here."),
            )
            .child(div().text_2xl().child(format!("Count: {}", self.count)))
            .child(
                div()
                    .id("increment")
                    .px_4()
                    .py_2()
                    .rounded_lg()
                    .bg(rgb(0x6366f1))
                    .hover(|style| style.bg(rgb(0x818cf8)))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count += 1;
                        cx.notify();
                    }))
                    .child("Increment"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("secondary-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Tilde".into(),
            items: vec![MenuItem::action("Quit Tilde", Quit)],
        }]);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let bounds = Bounds::centered(None, size(px(640.0), px(420.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Tilde".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Tilde::default()),
        )
        .expect("failed to open the application window");

        cx.activate(true);
    });
}
