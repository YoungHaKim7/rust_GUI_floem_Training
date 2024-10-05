use floem::{
    keyboard::{Key, Modifiers, NamedKey},
    reactive::{create_signal, SignalGet, SignalUpdate},
    unit::UnitExt,
    views::{dyn_view, Decorators, LabelClass, LabelCustomStyle},
    IntoView, View,
};

pub fn app() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);
    let view = (
        dyn_view(move || format!("Value: {}", counter.get())),
        counter.style(|s| s.padding(10.0)),
        ("Increment"
            .style(|s| s.border_radius(10.0))
            .on_click_stop({
                move |_| {
                    set_counter.update(|value| *value += 1);
                }
            })
            .keyboard_navigatable(),)
            .style(|s| {
                s.class(LabelClass, |s| {
                    s.apply(LabelCustomStyle::new().selectable(false).style())
                })
            }),
    )
        .style(|s| s.size(200.pct(), 200.pct()).items_center());

    let id = view.id();
    view.on_key_up(Key::Named(NamedKey::F5), Modifiers::empty(), move |_| {
        id.inspect()
    })
}
