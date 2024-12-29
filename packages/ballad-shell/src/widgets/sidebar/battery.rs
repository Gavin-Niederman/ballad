use ballad_services::battery::{BATTERY_SERVICE, BatteryService};
use gtk::glib::{clone, closure_local};
use gtk::prelude::{BoxExt, WidgetExt};
use gtk::{Box, Stack, StackTransitionType, prelude::ObjectExt};
use gtk::{Label, LevelBar, glib};
use typed_builder::TypedBuilder;

use crate::widgets::symbolic_icon::symbolic_icon;

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
    pub fn as_class_name(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Critical => "critical",
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

    let critical_icon = symbolic_icon("bat-critical-symbolic", 24);
    let low_icon = symbolic_icon("bat-low-symbolic", 24);
    let medium_icon = symbolic_icon("bat-medium-symbolic", 24);
    let high_icon = symbolic_icon("bat-high-symbolic", 24);
    let full_icon = symbolic_icon("bat-full-symbolic", 24);
    let charging_icon = symbolic_icon("bat-charging-symbolic", 24);

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
        .css_classes(["battery-bar", "vertical"])
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
                    Some(level.as_class_name())
                })
                .sync_create()
                .build();

            service
                .bind_property("percentage", &percent_label, "label")
                .transform_to(|_, percent: f64| Some(format!("{:.0}%", percent)))
                .sync_create()
                .build();

            fn bar_classes(battery: &BatteryService) -> Vec<&'static str> {
                let mut class_names = vec!["battery-bar", "vertical"];
                let percent = battery.percentage();
                let level = BatteryLevel::from_percent(percent);
                class_names.push(level.as_class_name());
                if battery.charging() {
                    class_names.push("charging");
                }
                class_names
            }

            service.connect_closure(
                "battery-changed",
                false,
                closure_local!(
                    #[weak]
                    battery_bar,
                    move |battery: BatteryService| {
                        let class_names = bar_classes(&battery);

                        battery_bar.set_css_classes(&class_names);
                        battery_bar.set_value(battery.percentage() / 100.0);
                    }
                ),
            );

            let class_names = bar_classes(service);
            battery_bar.set_css_classes(&class_names);
            battery_bar.set_value(service.percentage() / 100.0);
        }
    ));

    container.append(&icon_stack);
    container.append(&percent_label);
    container.append(&battery_bar);

    container
}
