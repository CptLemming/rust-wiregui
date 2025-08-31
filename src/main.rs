use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};

use crate::{app::WireGUI, state::StateModel};

mod app;
mod connections;
mod info;
mod net;
mod notify;
mod state;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        cx.activate(true);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                StateModel::init(cx);
                WireGUI::new(cx)
            },
        )
        .unwrap();
    });
}
