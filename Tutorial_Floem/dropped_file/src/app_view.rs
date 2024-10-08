use floem::{
    event::EventListener,
    keyboard::{Key, Modifiers, NamedKey},
    unit::UnitExt,
    views::{dyn_view, Decorators},
    IntoView, View,
};

pub fn app_view() -> impl IntoView {
    let view = dyn_view(move || "dropped file".to_string()).style(|s| {
        s.size(100.pct(), 100.pct())
            .flex_col()
            .items_center()
            .justify_center()
            .font_size(30)
    });

    let id = view.id();
    view.on_key_up(Key::Named(NamedKey::F5), Modifiers::empty(), move |_| {
        id.inspect()
    })
    .on_event_stop(EventListener::PointerMove, |x| {
        println!("PointerMove {:?}", x.point());
    })
    .on_event_stop(EventListener::DroppedFile, |x| {
        println!("DroppedFile {:?}", x);
    })
}
