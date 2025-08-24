use gpui::*;

use crate::{connections::ConnectionList, info::ConnectionInfo};

pub struct WireGUI {
    pub list_view: Entity<ConnectionList>,
    pub detail_view: Entity<ConnectionInfo>,
}

impl WireGUI {
    pub fn new(app: &mut App) -> Entity<Self> {
        let list_view = ConnectionList::new(app);
        let detail_view = ConnectionInfo::new(app);

        app.new(|_| Self {
            list_view,
            detail_view,
        })
    }
}

impl Render for WireGUI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // println!("Render : WireGUI");

        div()
            .gap_1()
            .bg(rgb(0x1b1b1b))
            .size_full()
            .flex()
            .flex_row()
            .text_color(rgb(0xb2b2b2))
            .child(
                div()
                    .w(AbsoluteLength::Pixels(px(300.0)))
                    .child(self.list_view.clone()),
            )
            .child(div().flex_auto().child(self.detail_view.clone()))
    }
}
