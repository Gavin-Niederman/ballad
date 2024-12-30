use gtk::Image;

pub fn symbolic_icon(icon_name: &str, size: i32) -> Image {
    Image::builder()
        .icon_name(icon_name)
        .pixel_size(size)
        .css_classes(["symbolic"])
        .build()
}
