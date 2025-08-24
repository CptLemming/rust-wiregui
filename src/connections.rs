use gpui::*;

use crate::state::StateModel;

pub struct ConnectionList {
    state: ListState,
}

impl ConnectionList {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let state = cx.global::<StateModel>().inner.clone();
            cx.subscribe(&state, |this: &mut ConnectionList, model, _event, cx| {
                // println!("Update");
                let items = model.read(cx).items.clone();
                this.state = ListState::new(items.len(), ListAlignment::Top, Pixels(20.));
                cx.notify();
            })
            .detach();

            // println!("Init : {}", state.read(cx).items.len());
            ConnectionList {
                state: ListState::new(state.read(cx).items.len(), ListAlignment::Top, Pixels(20.)),
            }
        })
    }
}

impl Render for ConnectionList {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0x2a2a2a)).flex().child(
            list(self.state.clone(), move |idx, _win, cx| {
                let state = cx.global::<StateModel>().inner.clone();
                // println!("Render : {} -> {:?}", idx, state.read(cx).selected);
                let item = state.read(cx).items.get(idx).unwrap().clone();
                div().child(item).into_any_element()
            })
            .w_full()
            .h_full(),
        )
    }
}

#[derive(Clone, Debug, IntoElement)]
pub struct ConnectionItem {
    pub name: SharedString,
    pub active: bool,
}

impl ConnectionItem {
    pub fn toggle(self: &mut Self, cx: &mut App) {
        StateModel::update(
            |state, cx| {
                state.toggle(self.name.clone(), cx);
            },
            cx,
        );
    }

    pub fn select(self: &mut Self, cx: &mut App) {
        StateModel::update(
            |state, cx| {
                state.select(self.name.clone(), cx);
            },
            cx,
        );
    }
}

impl RenderOnce for ConnectionItem {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .px_4()
            .py_1()
            .gap_3()
            .w_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .child(ConnectionState::new(self.active)),
            )
            .child(
                div()
                    .flex_col()
                    .child(div().child(self.name.clone()))
                    .child(div().text_color(rgb(0x7f7f7f)).child("never")),
            )
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x1c1c1c)))
            .on_mouse_down(MouseButton::Left, move |_, _, app| self.clone().select(app))
    }
}

#[derive(Clone, Debug, IntoElement)]
pub struct ConnectionState {
    pub active: bool,
}

impl ConnectionState {
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}

impl RenderOnce for ConnectionState {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .w_4()
            .h_4()
            .rounded_full()
            .bg(rgb(if self.active { 0x48bb78 } else { 0xc2c2c2 }))
    }
}
