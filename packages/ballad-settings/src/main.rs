use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::{AccessibleRole, Image};
use gtk::{
    Button, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, glib,
    style_context_add_provider_for_display,
};
use gtk::{Orientation, Paned, Stack, prelude::*};

mod pages;

fn main() {
    let app = gtk::Application::new(
        Some("com.gavinniederman.ballad-settings"),
        Default::default(),
    );

    app.connect_activate(build_ui);
    app.connect_startup(|_| {
        let provider = CssProvider::new();
        provider.load_from_string(include_str!("../style/style.css"));
        style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.run();
}

struct Page {
    name: &'static str,
    icon_name: &'static str,
    title: &'static str,
}

const PAGES: &[Page] = &[
    Page {
        title: "Shell",
        icon_name: "preferences-desktop-theme-symbolic",
        name: "shell",
    },
    Page {
        title: "User",
        icon_name: "system-users-symbolic",
        name: "user",
    },
];

fn page_switcher(stack: &Stack) -> gtk::Box {
    let button_map: Rc<RefCell<HashMap<String, Button>>> = Default::default();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["page-switcher"])
        .name("page-switcher")
        .spacing(12)
        .accessible_role(AccessibleRole::TabList)
        .build();

    for Page {
        name,
        icon_name,
        title,
    } in PAGES
    {
        let button = gtk::Button::builder()
            .name("page-switcher-button")
            .accessible_role(AccessibleRole::Tab)
            .build();

        button_map
            .borrow_mut()
            .insert(name.to_string(), button.clone());

        let button_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["page-switcher-button-content"])
            .name("page-switcher-button-content")
            .build();

        let icon = Image::builder()
            .icon_name(*icon_name)
            .pixel_size(24)
            .build();

        button_content.append(&icon);
        button_content.append(&gtk::Label::builder().label(*title).build());

        button.set_child(Some(&button_content));
        container.append(&button);

        button.connect_clicked(clone!(
            #[weak]
            stack,
            move |_| {
                stack.set_visible_child_name(name);
            }
        ));
    }

    for (name, button) in button_map.borrow().iter() {
        if *name == stack.visible_child_name().unwrap() {
            button.add_css_class("selected");
        }
    }

    stack.connect_visible_child_name_notify(move |stack| {
        let visible_child = stack.visible_child_name().unwrap().to_string();
        for (name, button) in button_map.borrow().iter() {
            if *name == visible_child {
                button.add_css_class("selected");
            } else {
                button.remove_css_class("selected");
            }
        }
    });

    container
}

fn build_ui(app: &gtk::Application) {
    const DEFAULT_WIDTH: i32 = 800;

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Ballad Settings")
        .default_height(600)
        .default_width(DEFAULT_WIDTH)
        .build();

    let stack = pages::settings_stack();

    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .name("settings-paned")
        .start_child(&page_switcher(&stack))
        .end_child(&stack)
        .position(DEFAULT_WIDTH / 4)
        .build();

    window.set_child(Some(&paned));
    window.present();
}
