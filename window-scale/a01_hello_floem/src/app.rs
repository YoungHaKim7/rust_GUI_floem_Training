use floem::{
    reactive::create_signal,
    views::{button, label, Decorators},
    IntoView,
};

pub fn app() -> impl IntoView {
    let (counter, mut set_counter) = create_signal(0);

    (
        label(move || format!("VAlue: {counter}")),
        (
            button("Increment").action(move || set_counter += 1),
            button("Decrement").action(move || set_counter -= 1),
        ),
    )
        .style(|s| s.flex_col())
}
