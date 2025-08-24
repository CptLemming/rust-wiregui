use gpui::*;

use crate::{connections::ConnectionItem, state::StateModel};

pub struct ConnectionInfo {
    selected: Option<ConnectionItem>,
}

impl ConnectionInfo {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let state = cx.global::<StateModel>().inner.clone();
            cx.subscribe(&state, |this: &mut ConnectionInfo, model, _event, cx| {
                this.selected = model.read(cx).get_selected();
                cx.notify();
            })
            .detach();

            ConnectionInfo {
                selected: state.read(cx).get_selected(),
            }
        })
    }
}

impl Render for ConnectionInfo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(match &self.selected {
            Some(item) => div()
                .size_full()
                .flex()
                .flex_col()
                .child(ConnectionDetail::new(item.clone())),
            None => div().size_full().child(NoConnection::new()),
        })
    }
}

#[derive(Clone, Debug, IntoElement)]
struct NoConnection {}

impl NoConnection {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for NoConnection {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(div().child("No connection selected"))
    }
}

#[derive(Clone, Debug, IntoElement)]
struct ConnectionDetail {
    item: ConnectionItem,
}

impl ConnectionDetail {
    pub fn new(item: ConnectionItem) -> Self {
        Self { item }
    }
}

impl RenderOnce for ConnectionDetail {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let item = self.item.clone();

        div()
            .flex()
            .flex_col()
            .p_6()
            .m_6()
            .flex_1()
            .bg(rgb(0x2a2a2a))
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight(500.))
                    .child(self.item.name),
            )
            .child(
                div()
                    .flex_auto()
                    .bg(rgb(0x1b1b1b))
                    .p_4()
                    .child("<profile goes here>"),
            )
            .child(
                div()
                    .flex()
                    .py_3()
                    .gap_3()
                    .justify_end()
                    .child(
                        div()
                            .bg(rgb(0x3b3b3b))
                            .p_2()
                            .rounded_sm()
                            .cursor_pointer()
                            .font_weight(FontWeight(500.))
                            .child("Delete"),
                    )
                    .child(
                        div()
                            .bg(rgb(0xf4752f))
                            .p_2()
                            .rounded_sm()
                            .text_color(rgb(0x282429))
                            .font_weight(FontWeight(500.))
                            .cursor_pointer()
                            .hover(|style| style.bg(rgb(0xd85108)))
                            .child(if self.item.active {
                                "Deactivate"
                            } else {
                                "Activate"
                            })
                            .on_mouse_down(MouseButton::Left, move |_, _, app| {
                                item.clone().toggle(app)
                            }),
                    )
                    .child(
                        div()
                            .bg(rgb(0xa6e3b8))
                            .p_2()
                            .rounded_sm()
                            .text_color(rgb(0x282429))
                            .font_weight(FontWeight(500.))
                            .cursor_pointer()
                            .child("Save"),
                    ),
            )
    }
}
