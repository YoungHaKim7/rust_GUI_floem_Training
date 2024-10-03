use std::path::Path;

use floem::{kurbo::Size, window::WindowConfig, Application};

use crate::app::app_view::load_png_svg_icon::load_png_icon;
use app::app_view::app_view;
mod app;

fn main() {
    let png_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/ferris.png");
    let png_icon = load_png_icon(Path::new(png_icon_path));

    Application::new()
        .window(
            |_| app_view(),
            Some(
                WindowConfig::default()
                    .size(Size::new(800.0, 250.0))
                    .title("Window Icon Example")
                    .window_icon(png_icon),
            ),
        )
        .run();
}
