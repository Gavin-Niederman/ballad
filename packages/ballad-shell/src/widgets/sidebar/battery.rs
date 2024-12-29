use ballad_services::battery::{BATTERY_SERVICE, BatteryService};
use gtk::glib::{clone, closure_local};
use gtk::prelude::BoxExt;
use gtk::{Box, Stack, StackTransitionType, prelude::ObjectExt};
use gtk::{Image, Label, LevelBar, glib};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryLevel {
    Full,
    High,
    Medium,
    Low,
    Critical,
}
impl BatteryLevel {
    pub fn from_percent(percent: f64) -> Self {
        if percent >= 98.0 {
            Self::Full
        } else if percent > 60.0 {
            Self::High
        } else if percent > 40.0 {
            Self::Medium
        } else if percent > 10.0 {
            Self::Low
        } else {
            Self::Critical
        }
    }
}

#[derive(Debug, TypedBuilder, Clone, PartialEq, Eq)]
pub struct BatteryProperties {
    #[builder(default = crate::widgets::Orientation::Vertical)]
    orientation: crate::widgets::Orientation,
}

pub fn battery(BatteryProperties { orientation }: BatteryProperties) -> Box {
    let container = Box::builder()
        .orientation(orientation.clone().into())
        .css_classes(["battery"])
        .build();

    let icon_stack = Stack::builder()
        .transition_type(StackTransitionType::SlideUp)
        .build();

    let critical_icon = Image::builder()
        .icon_name("bat-critical-symbolic")
        .pixel_size(24)
        .build();
    let low_icon = Image::builder()
        .icon_name("bat-low-symbolic")
        .pixel_size(24)
        .build();
    let medium_icon = Image::builder()
        .icon_name("bat-medium-symbolic")
        .pixel_size(24)
        .build();
    let high_icon = Image::builder()
        .icon_name("bat-high-symbolic")
        .pixel_size(24)
        .build();
    let full_icon = Image::builder()
        .icon_name("bat-full-symbolic")
        .pixel_size(24)
        .build();
    let charging_icon = Image::builder()
        .icon_name("bat-charging-symbolic")
        .pixel_size(24)
        .build();

    icon_stack.add_named(&critical_icon, Some("critical"));
    icon_stack.add_named(&low_icon, Some("low"));
    icon_stack.add_named(&medium_icon, Some("medium"));
    icon_stack.add_named(&high_icon, Some("high"));
    icon_stack.add_named(&full_icon, Some("full"));
    icon_stack.add_named(&charging_icon, Some("charging"));
    icon_stack.set_visible_child_name("critical");

    let percent_label = Label::builder().css_classes(["percent-display"]).build();

    let battery_bar = LevelBar::builder()
        .orientation(orientation.into())
        .inverted(true)
        .mode(gtk::LevelBarMode::Continuous)
        .build();

    BATTERY_SERVICE.with(clone!(
        #[weak]
        icon_stack,
        #[weak]
        percent_label,
        #[weak]
        battery_bar,
        move |service| {
            service
                .bind_property("percentage", &icon_stack, "visible-child-name")
                .transform_to(|_, percent: f64| {
                    let level = BatteryLevel::from_percent(percent);
                    match level {
                        BatteryLevel::Full => Some("full"),
                        BatteryLevel::High => Some("high"),
                        BatteryLevel::Medium => Some("medium"),
                        BatteryLevel::Low => Some("low"),
                        BatteryLevel::Critical => Some("critical"),
                    }
                })
                .sync_create()
                .build();

            service
                .bind_property("percentage", &percent_label, "label")
                .transform_to(|_, percent: f64| Some(format!("{:.0}%", percent)))
                .sync_create()
                .build();

            service.connect_closure(
                "battery-changed",
                false,
                closure_local!(move |battery: BatteryService| {
                    let mut class_names = Vec::new();
                    let percent = battery.percentage();
                    let level = BatteryLevel::from_percent(percent);
                    match level {
                        BatteryLevel::Full => {
                            class_names.push("full");
                        }
                        BatteryLevel::High => {
                            class_names.push("high");
                        }
                        BatteryLevel::Medium => {
                            class_names.push("medium");
                        }
                        BatteryLevel::Low => {
                            class_names.push("low");
                        }
                        BatteryLevel::Critical => {
                            class_names.push("critical");
                        }
                    }
                    if battery.charging() {
                        class_names.push("charging");
                    }

                    battery_bar.set_value(percent / 100.0);
                }),
            );
        }
    ));

    container.append(&icon_stack);
    container.append(&percent_label);
    container.append(&battery_bar);

    container
}
