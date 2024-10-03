use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    kurbo::Size,
    new_window,
    views::{button, label, v_stack, Decorators},
    window::WindowConfig,
    IntoView, View,
};

use load_png_svg_icon::load_svg_icon;
use sub_window_view::sub_window_view;

pub mod load_png_svg_icon;
pub mod sub_window_view;

pub fn app_view() -> impl IntoView {
    let view = v_stack((
        label(move || String::from("This window has an icon from a PNG file"))
            .style(|s| s.font_size(30.0)),
        button("Open another window").action(|| {
            let svg_icon = load_svg_icon(include_str!("../../assets/ferris.svg"));
            new_window(
                sub_window_view,
                Some(
                    WindowConfig::default()
                        .size(Size::new(600.0, 150.0))
                        .title("Window Icon Sub Example")
                        .window_icon(svg_icon),
                ),
            );
        }),
    ))
    .style(|s| {
        s.flex_col()
            .items_center()
            .justify_center()
            .width_full()
            .height_full()
            .column_gap(10.0)
    });

    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let Event::KeyUp(e) = e {
            if e.key.logical_key == Key::Named(NamedKey::F5) {
                id.inspect();
            }
        }
    })
}
