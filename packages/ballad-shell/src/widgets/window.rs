use gtk::{gdk::Monitor, prelude::GtkWindowExt, Application, ApplicationWindow};
use gtk4_layer_shell::LayerShell;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Top,
    Bottom,
    Left,
    Right,
}
impl From<Anchor> for gtk4_layer_shell::Edge {
    fn from(anchor: Anchor) -> Self {
        match anchor {
            Anchor::Top => gtk4_layer_shell::Edge::Top,
            Anchor::Bottom => gtk4_layer_shell::Edge::Bottom,
            Anchor::Left => gtk4_layer_shell::Edge::Left,
            Anchor::Right => gtk4_layer_shell::Edge::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Top,
    Bottom,
    Overlay,
}
impl From<Layer> for gtk4_layer_shell::Layer {
    fn from(layer: Layer) -> Self {
        match layer {
            Layer::Top => gtk4_layer_shell::Layer::Top,
            Layer::Bottom => gtk4_layer_shell::Layer::Bottom,
            Layer::Overlay => gtk4_layer_shell::Layer::Overlay,
        }
    }
}

#[derive(Debug, TypedBuilder, Clone, PartialEq, Eq)]
#[builder(field_defaults(default))]
pub struct WindowProperties<'a> {
    #[builder(setter(strip_option))]
    pub title: Option<&'a str>,
    #[builder(default = &[Anchor::Top, Anchor::Bottom, Anchor::Left, Anchor::Right])]
    pub anchors: &'a [Anchor],
    #[builder(default = Layer::Top)]
    pub layer: Layer,
    pub auto_exclusive: bool,

    #[builder(!default)]
    pub monitor: Monitor,

    #[builder(!default)]
    pub application: &'a Application,
}

pub fn window(
    WindowProperties {
        title,
        anchors,
        layer,
        application,
        auto_exclusive,
        monitor
    }: WindowProperties<'_>,
) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(application)
        .build();
    window.init_layer_shell();

    window.set_monitor(&monitor);
    window.set_title(title);
    window.set_layer(layer.into());

    if auto_exclusive {
        window.auto_exclusive_zone_enable();
    }

    for anchor in anchors {
        window.set_anchor((*anchor).into(), true);
    }

    window
}
