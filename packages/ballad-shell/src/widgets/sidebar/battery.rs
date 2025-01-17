use ballad_services::upower::{UPOWER_SERVICE, UPowerService};
use gtk::glib::{clone, closure_local};
use gtk::prelude::{BoxExt, WidgetExt};
use gtk::{Box, Stack, StackTransitionType, prelude::ObjectExt};
use gtk::{Label, LevelBar, glib};
use typed_builder::TypedBuilder;

use crate::widgets::icon::symbolic_icon;

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
        if percent >= 99.0 {
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
#[builder(build_method(into = Box))]
pub struct Battery {
    #[builder(default = crate::widgets::Orientation::Vertical)]
    pub orientation: crate::widgets::Orientation,
}
impl From<Battery> for Box {
    fn from(props: Battery) -> Self {
        battery(props)
    }
}

pub fn battery(Battery { orientation }: Battery) -> Box {
    let container = Box::builder()
        .orientation(orientation.into())
        .name("battery-container")
        .css_classes(["battery"])
        .build();

    let icon_stack = Stack::builder()
        .transition_type(StackTransitionType::SlideUp)
        .name("battery-icon-stack")
        .build();

    let critical_icon = symbolic_icon("bat-critical-symbolic", 24);
    critical_icon.set_widget_name("battery-critical-icon");
    let low_icon = symbolic_icon("bat-low-symbolic", 24);
    low_icon.set_widget_name("battery-low-icon");
    let medium_icon = symbolic_icon("bat-medium-symbolic", 24);
    medium_icon.set_widget_name("battery-medium-icon");
    let high_icon = symbolic_icon("bat-high-symbolic", 24);
    high_icon.set_widget_name("battery-high-icon");
    let full_icon = symbolic_icon("bat-full-symbolic", 24);
    full_icon.set_widget_name("battery-full-icon");
    let charging_icon = symbolic_icon("bat-charging-symbolic", 24);
    charging_icon.set_widget_name("battery-charging-icon");

    icon_stack.add_named(&critical_icon, Some("critical"));
    icon_stack.add_named(&low_icon, Some("low"));
    icon_stack.add_named(&medium_icon, Some("medium"));
    icon_stack.add_named(&high_icon, Some("high"));
    icon_stack.add_named(&full_icon, Some("full"));
    icon_stack.add_named(&charging_icon, Some("charging"));

    let percent_label = Label::builder()
        .name("battery-percent-display")
        .css_classes(["percent-display"])
        .build();

    let battery_bar_classes = if orientation == crate::widgets::Orientation::Horizontal {
        ["battery-bar", "horizontal"]
    } else {
        ["battery-bar", "vertical"]
    };

    let battery_bar = LevelBar::builder()
        .orientation(orientation.into())
        .css_classes(battery_bar_classes)
        .name("battery-bar")
        .inverted(true)
        .mode(gtk::LevelBarMode::Continuous)
        .build();

    UPOWER_SERVICE.with(clone!(
        #[weak]
        icon_stack,
        #[weak]
        percent_label,
        #[weak]
        battery_bar,
        move |service| {
            service
                .bind_property("percentage", &percent_label, "label")
                .transform_to(|_, percent: f64| Some(format!("{:.0}%", percent)))
                .sync_create()
                .build();

            fn bar_classes(battery: &UPowerService) -> Vec<&'static str> {
                let mut class_names = vec!["battery-bar", "vertical"];
                let percent = battery.percentage();
                let level = BatteryLevel::from_percent(percent);
                class_names.push(level.as_class_name());
                if battery.charging() {
                    class_names.push("charging");
                }
                class_names
            }

            fn shown_battery_icon(level: BatteryLevel, charging: bool) -> &'static str {
                if charging && level != BatteryLevel::Full {
                    "charging"
                } else {
                    match level {
                        BatteryLevel::Full => "full",
                        BatteryLevel::High => "high",
                        BatteryLevel::Medium => "medium",
                        BatteryLevel::Low => "low",
                        BatteryLevel::Critical => "critical",
                    }
                }
            }

            service.connect_closure(
                "battery-changed",
                false,
                closure_local!(
                    #[weak]
                    battery_bar,
                    #[weak]
                    icon_stack,
                    move |battery: UPowerService| {
                        let class_names = bar_classes(&battery);

                        battery_bar.set_css_classes(&class_names);
                        battery_bar.set_value(battery.percentage() / 100.0);

                        if battery.charging() {
                            icon_stack.set_transition_type(StackTransitionType::SlideDown);
                        } else {
                            icon_stack.set_transition_type(StackTransitionType::SlideUp);
                        }

                        icon_stack.set_visible_child_name(shown_battery_icon(
                            BatteryLevel::from_percent(battery.percentage()),
                            battery.charging(),
                        ));
                    }
                ),
            );

            let class_names = bar_classes(service);
            battery_bar.set_css_classes(&class_names);
            battery_bar.set_value(service.percentage() / 100.0);
            icon_stack.set_visible_child_name(shown_battery_icon(
                BatteryLevel::from_percent(service.percentage()),
                service.charging(),
            ));
        }
    ));

    container.append(&icon_stack);
    container.append(&percent_label);
    container.append(&battery_bar);

    container
}
