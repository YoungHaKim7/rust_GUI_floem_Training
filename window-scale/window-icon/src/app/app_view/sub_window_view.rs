use floem::{
    close_window,
    views::{button, label, v_stack, Decorators},
    window::WindowId,
    IntoView,
};

pub fn sub_window_view(id: WindowId) -> impl IntoView {
    v_stack((
        label(move || String::from("This window has an icon from an SVG file."))
            .style(|s| s.font_size(30.0)),
        button("Close this window").action(move || close_window(id)),
    ))
    .style(|s| {
        s.flex_col()
            .items_center()
            .justify_center()
            .width_full()
            .height_full()
            .column_gap(10.0)
    })
}
