use gpui::*;

use crate::{
    connections::ConnectionItem,
    net::{activate_connection, deactivate_connection, get_connection_list},
    notify::send_notification,
};

#[derive(Clone)]
pub struct State {
    pub selected: Option<SharedString>,
    pub items: Vec<ConnectionItem>,
}

impl State {
    pub fn get_selected(&self) -> Option<ConnectionItem> {
        if let Some(ref name) = self.selected {
            return self.items.iter().find(|item| &item.name == name).cloned();
        }

        None
    }
}

#[derive(Clone)]
pub struct StateModel {
    pub inner: Entity<State>,
}

impl StateModel {
    pub fn init(app: &mut App) {
        let items = get_connection_list();

        let model = app.new(|_cx| State {
            selected: None,
            // selected: Some("home".into()),
            items,
        });
        let this = Self { inner: model };
        app.set_global(this.clone());
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut App), cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }

    pub fn select(&self, name: SharedString, cx: &mut App) {
        println!("Select : {name}");
        self.inner.update(cx, |model, cx| {
            model.selected = Some(name);

            cx.emit(ListChangedEvent {});
        });
    }

    pub fn activate(&self, name: SharedString, cx: &mut App) {
        println!("Activate : State : {name}");
        self.inner.update(cx, |model, cx| {
            if let Some(entry) = model.items.iter_mut().find(|entry| entry.name == name) {
                if !entry.active {
                    let _ = activate_connection(name);
                    entry.active = true;
                }
            }

            cx.emit(ListChangedEvent {});
        });
    }

    pub fn deactivate(&self, name: SharedString, cx: &mut App) {
        println!("Deactivate : State : {name}");
        self.inner.update(cx, |model, cx| {
            if let Some(entry) = model.items.iter_mut().find(|entry| entry.name == name) {
                if entry.active {
                    let _ = deactivate_connection(name);
                    entry.active = false;
                }
            }

            cx.emit(ListChangedEvent {});
        });
    }

    pub fn toggle(&self, name: SharedString, cx: &mut App) {
        println!("Toggle : State : {name}");
        self.inner.update(cx, |model, cx| {
            for entry in model.items.iter_mut() {
                if entry.name == name {
                    if entry.active {
                        let _ = deactivate_connection(name.clone());
                        let _ = send_notification(name.clone(), false);
                        entry.active = false;
                    } else {
                        let _ = activate_connection(name.clone());
                        let _ = send_notification(name.clone(), true);
                        entry.active = true;
                    }
                } else {
                    if entry.active {
                        let _ = deactivate_connection(entry.name.clone());
                        let _ = send_notification(entry.name.clone(), false);
                    }
                    entry.active = false;
                }
            }

            cx.emit(ListChangedEvent {});
        });
    }
}

impl Global for StateModel {}

#[derive(Clone, Debug)]
pub struct ListChangedEvent {}

impl EventEmitter<ListChangedEvent> for State {}
