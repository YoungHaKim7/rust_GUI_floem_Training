use std::path::Path;

use floem::window::Icon;

pub fn load_png_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

pub fn load_svg_icon(svg: &str) -> Icon {
    let svg = nsvg::parse_str(svg, nsvg::Units::Pixel, 96.0).unwrap();
    let (icon_width, icon_height, icon_rgba) = svg.rasterize_to_raw_rgba(1.0).unwrap();
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
