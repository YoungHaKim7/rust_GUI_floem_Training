use floem::{kurbo::Size, window::WindowConfig, Application};

use crate::app_view::app_view;

mod app_view;

fn main() {
    Application::new()
        .window(
            |_| app_view(),
            Some(
                WindowConfig::default()
                    .size(Size::new(800.0, 250.0))
                    .title("Window Size Example"),
            ),
        )
        .run();
}
