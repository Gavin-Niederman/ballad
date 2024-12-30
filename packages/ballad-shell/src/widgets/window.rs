use gtk::{Application, ApplicationWindow, gdk::Monitor, prelude::GtkWindowExt};
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
    Overlay,
    Top,
    Bottom,
    Background,
}
impl From<Layer> for gtk4_layer_shell::Layer {
    fn from(layer: Layer) -> Self {
        match layer {
            Layer::Top => gtk4_layer_shell::Layer::Top,
            Layer::Bottom => gtk4_layer_shell::Layer::Bottom,
            Layer::Overlay => gtk4_layer_shell::Layer::Overlay,
            Layer::Background => gtk4_layer_shell::Layer::Background,
        }
    }
}

#[derive(Debug, TypedBuilder, Clone, PartialEq, Eq)]
#[builder(field_defaults(default), build_method(into))]
pub struct LayershellWindow<'a> {
    #[builder(setter(strip_option))]
    pub title: Option<&'a str>,
    #[builder(default = &[Anchor::Top, Anchor::Bottom, Anchor::Left, Anchor::Right])]
    pub anchors: &'a [Anchor],
    #[builder(default = Layer::Top)]
    pub layer: Layer,
    pub auto_exclusive: bool,

    #[builder(setter(strip_option))]
    pub monitor: Option<Monitor>,

    #[builder(!default)]
    pub application: &'a Application,
}

impl From<LayershellWindow<'_>> for ApplicationWindow {
    fn from(properties: LayershellWindow) -> Self {
        layershell_window(properties)
    }
}

pub fn layershell_window(
    LayershellWindow {
        title,
        anchors,
        layer,
        application,
        auto_exclusive,
        monitor,
    }: LayershellWindow<'_>,
) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(application)
        .build();
    window.init_layer_shell();

    if let Some(monitor) = monitor {
        window.set_monitor(&monitor);
    }
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
