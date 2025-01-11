use gtk::Image;

pub fn symbolic_icon(icon_name: &str, size: i32) -> Image {
    Image::builder()
        .icon_name(icon_name)
        .pixel_size(size)
        .css_classes(["symbolic"])
        .build()
}

pub fn app_icon(app_id: Option<&str>, size: i32) -> Image {
    Image::builder()
        .icon_name(app_id.unwrap_or("application-x-executable"))
        .pixel_size(size)
        .css_classes(["app-icon"])
        .build()
}
