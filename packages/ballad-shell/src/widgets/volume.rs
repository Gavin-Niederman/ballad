use std::cell::LazyCell;

use ballad_services::audio::AUDIO_SERVICE;
use gtk::{
    Align, Button, Stack, StackTransitionType,
    glib::{self, Propagation, clone},
    prelude::*,
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

pub fn volume(
    Volume {
        orientation,
        draw_value,
    }: Volume,
) -> gtk::Box {
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

    let service = AUDIO_SERVICE.with(|service| LazyCell::force(service).clone());

    smol::block_on(async {
        service.connect_muted(clone!(
            #[weak]
            icon_stack,
            #[weak]
            volume_bar,
            move |_, muted| {
                set_class_on_widget(muted, &volume_bar, "muted");
                icon_stack.set_visible_child_name(if muted { "muted" } else { "unmuted" });
            }
        ));
        service.connect_volume(clone!(
            #[weak]
            percent_display,
            #[weak]
            volume_bar,
            move |service, _| {
                let volume = service.volume_blocking();
                percent_display.set_label(&format!("{:.0}%", volume * 100.0));
                volume_bar.set_value(volume);
            }
        ));

        // User input
        volume_bar.connect_value_changed(clone!(
            #[weak]
            service,
            move |bar| {
                service.set_volume_blocking(bar.value());
            }
        ));
        mute_toggle.connect_clicked(clone!(
            #[weak]
            service,
            move |_| {
                let muted = service.muted_blocking();
                service.set_muted_blocking(!muted);
            }
        ));

        percent_display.set_label(&format!("{:.0}%", service.volume().await * 100.0));
        volume_bar.set_value(service.volume().await);
    });

    volume_container.append(&mute_toggle);
    if draw_value {
        volume_container.append(&percent_display);
    }
    volume_container.append(&volume_bar);

    volume_container
}
