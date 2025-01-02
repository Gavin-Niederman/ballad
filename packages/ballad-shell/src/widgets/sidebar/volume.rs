use std::cell::LazyCell;

use ballad_services::audio::AUDIO_SERVICE;
use gtk::{
    glib::{self, clone}, prelude::*, Button, Stack, StackTransitionType
};
use typed_builder::TypedBuilder;

use crate::widgets::symbolic_icon::symbolic_icon;

#[derive(Debug, TypedBuilder, Clone, PartialEq, Eq)]
#[builder(build_method(into = gtk::Box))]
pub struct Volume {
    #[builder(default = crate::widgets::Orientation::Vertical)]
    pub orientation: crate::widgets::Orientation,
}
impl From<Volume> for gtk::Box {
    fn from(props: Volume) -> Self {
        volume(props)
    }
}

pub fn volume(Volume { orientation }: Volume) -> gtk::Box {
    let volume_container = gtk::Box::builder()
        .orientation(orientation.clone().into())
        .name("volume-container")
        .css_classes(["volume"])
        .build();

    let unmuted_icon = symbolic_icon("speaker-on-symbolic", 24);
    let muted_icon = symbolic_icon("speaker-off-symbolic", 24);

    let icon_stack = Stack::builder()
        .transition_type(StackTransitionType::SlideUp)
        .name("volume-icon-stack")
        .build();
    icon_stack.add_named(&unmuted_icon, Some("unmuted"));
    icon_stack.add_named(&muted_icon, Some("muted"));

    let mute_toggle = Button::builder()
        .name("quick-settings-toggle")
        .css_classes(["icon-container", "hoverable"])
        .child(&icon_stack)
        .build();

    let percent_display = gtk::Label::builder()
        .name("volume-percent-display")
        .css_classes(["percent-display"])
        .build();

    let volume_bar_classes = if orientation == crate::widgets::Orientation::Horizontal {
        ["volume-bar", "horizontal"]
    } else {
        ["volume-bar", "vertical"]
    };

    let volume_bar = gtk::Scale::builder()
        .orientation(orientation.into())
        .css_classes(volume_bar_classes)
        .name("volume-bar")
        .build();

    AUDIO_SERVICE.with(clone!(
        #[weak]
        percent_display,
        #[weak]
        volume_bar,
        #[weak]
        mute_toggle,
        move |service| {
            let service = LazyCell::force(service).clone();

            service
                .bind_property("volume", &percent_display, "label")
                .transform_to(|_, value: f64| Some(format!("{:.0}%", value * 100.0)))
                .build();
            service
                .bind_property("muted", &icon_stack, "visible-child-name")
                .transform_to(|_, value: bool| Some(if value { "muted" } else { "unmuted" }))
                .build();
            // service
            //     .bind_property("volume", &volume_bar, "fill-value")
            //     .transform_to(|_, value: f64| Some(value * 100.0))
            //     .build();

            percent_display.set_label(&format!("{:.0}%", service.volume() * 100.0));
            volume_bar.set_value(service.volume());

            mute_toggle.connect_clicked(move |_| {
                service.set_muted(!service.muted());
            });

        }
    ));

    volume_container.append(&mute_toggle);
    volume_container.append(&percent_display);
    volume_container.append(&volume_bar);

    volume_container
}
