use std::cell::LazyCell;

use ballad_services::audio::{AUDIO_SERVICE, AudioService};
use gtk::{
    glib::{self, clone, closure_local}, prelude::*, Align, Button, Stack, StackTransitionType
};
use typed_builder::TypedBuilder;

use crate::{utils::set_class_on_widget, widgets::icon::symbolic_icon};

#[derive(Debug, TypedBuilder, Clone, PartialEq, Eq)]
#[builder(build_method(into = gtk::Box))]
pub struct Volume {
    #[builder(default = crate::widgets::Orientation::Vertical)]
    pub orientation: crate::widgets::Orientation,
    #[builder(default = true)]
    pub draw_value: bool,
}
impl From<Volume> for gtk::Box {
    fn from(props: Volume) -> Self {
        volume(props)
    }
}

pub fn volume(Volume { orientation, draw_value }: Volume) -> gtk::Box {
    let volume_container = gtk::Box::builder()
        .orientation(orientation.into())
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
    volume_bar.set_range(0.0, 1.0);

    if orientation == crate::widgets::Orientation::Horizontal {
        volume_container.set_valign(Align::Center);
        volume_container.set_hexpand(true);
        volume_bar.set_hexpand(true);
        volume_container.set_spacing(4);
        percent_display.set_vexpand(true);
    } else {
        volume_bar.set_inverted(true);
    }

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

            service.connect_closure(
                "audio-changed",
                false,
                closure_local!(
                    #[weak]
                    volume_bar,
                    move |service: AudioService| {
                        volume_bar.set_value(service.volume());
                        set_class_on_widget(service.muted(), &volume_bar, "muted");
                    }
                ),
            );

            // User input
            volume_bar.connect_value_changed(clone!(
                #[weak]
                service,
                move |bar| service.set_volume(bar.value())
            ));
            mute_toggle.connect_clicked(clone!(
                #[weak]
                service,
                move |_| {
                    service.set_muted(!service.muted());
                    service.emit_by_name::<()>("audio-changed", &[]);
                }
            ));

            percent_display.set_label(&format!("{:.0}%", service.volume() * 100.0));
            volume_bar.set_value(service.volume());
        }
    ));

    volume_container.append(&mute_toggle);
    if draw_value {
        volume_container.append(&percent_display);
    }
    volume_container.append(&volume_bar);

    volume_container
}
