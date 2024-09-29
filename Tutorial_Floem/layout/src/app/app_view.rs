use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    kurbo::Size,
    new_window,
    style::FontSize,
    taffy::AlignContent,
    views::{button, container, h_stack, label, v_stack, Decorators},
    window::WindowConfig,
    IntoView, View,
};

pub mod draggable_sidebar;
pub mod holy_grail;
pub mod left_sidebar;
pub mod right_sidebar;
pub mod tab_navigation;

fn list_item<V: IntoView + 'static>(name: String, view_fn: impl Fn() -> V) -> impl IntoView {
    h_stack((
        label(move || name.clone()).style(|s| s),
        container(view_fn()).style(|s| s.width_full().justify_content(AlignContent::End)),
    ))
    .style(|s| s.width(200))
}

pub fn app_view() -> impl IntoView {
    let view = v_stack((
        label(move || String::from("Static layouts"))
            .style(|s| s.font_size(40.0).margin_bottom(15.0)),
        list_item(String::from("Left sidebar"), move || {
            button("Open").action(|| {
                new_window(
                    |_| left_sidebar::left_sidebar_view(),
                    Some(
                        WindowConfig::default()
                            .size(Size::new(700.0, 400.0))
                            .title("Left sidebar"),
                    ),
                );
            })
        })
        .style(|s| s.font_size(22)),
        list_item(String::from("Right sidebar"), move || {
            button("Open").action(|| {
                new_window(
                    |_| right_sidebar::right_sidebar_view(),
                    Some(
                        WindowConfig::default()
                            .size(Size::new(700.0, 400.0))
                            .title("Right sidebar"),
                    ),
                );
            })
        })
        .style(|s| s.font_size(22)),
        list_item(String::from("Holy grail"), move || {
            button("Open").action(|| {
                new_window(
                    |_| holy_grail::holy_grail_view(),
                    Some(
                        WindowConfig::default()
                            .size(Size::new(700.0, 400.0))
                            .title("Holy Grail"),
                    ),
                );
            })
        })
        .style(|s| s.font_size(22)),
        label(move || String::from("Interactive layouts"))
            .style(|s| s.font_size(40.0).margin_top(15.0).margin_bottom(15.0)),
        list_item(String::from("Tab navigation"), move || {
            button("Open").action(|| {
                new_window(
                    |_| tab_navigation::tab_navigation_view(),
                    Some(
                        WindowConfig::default()
                            .size(Size::new(400.0, 250.0))
                            .title("Tab navigation"),
                    ),
                );
            })
        })
        .style(|s| s.font_size(22)),
        list_item(String::from("Draggable sidebar"), move || {
            button("Open").action(|| {
                new_window(
                    |_| draggable_sidebar::draggable_sidebar_view(),
                    Some(
                        WindowConfig::default()
                            .size(Size::new(700.0, 400.0))
                            .title("Draggable sidebar"),
                    ),
                );
            })
        })
        .style(|s| s.font_size(22)),
    ))
    .style(|s| {
        s.flex_col()
            .width_full()
            .height_full()
            .padding(10.0)
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
    .window_title(|| String::from("Layout examples"))
}
