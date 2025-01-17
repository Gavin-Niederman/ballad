use gtk::{Align, Label, Widget, prelude::*};
use typed_builder::TypedBuilder;

pub mod user;
pub mod shell;

pub fn settings_stack() -> gtk::Stack {
    let stack = gtk::Stack::builder().name("settings-stack").build();

    stack.add_titled(&shell::shell_page(), Some("shell"), "Shell");
    stack.add_titled(&user::user_page(), Some("user"), "User");

    stack
}

fn option<O: IsA<Widget>>(name: &str, description: Option<&str>, option: &O) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["option"])
        .name(name)
        .hexpand(true)
        .build();
    let text_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["option-text"])
        .name("option-text")
        .hexpand(true)
        .halign(Align::Start)
        .build();

    text_container.append(&Label::builder().label(name).css_classes(["option-name"]).halign(Align::Start).build());
    if let Some(description) = description {
        text_container.append(
            &Label::builder()
                .label(description)
                .css_classes(["subtext", "option-subtext"])
                .halign(Align::Start)
                .build(),
        );
    }

    container.append(&text_container);
    option.set_halign(Align::End);
    container.append(option);

    container
}

#[derive(TypedBuilder)]
#[builder(
    mutators(
        fn with_option(&mut self, option: &C) {
            self.options.push(option.clone());
        }
    ),
    build_method(into = gtk::Box)
)]
pub struct Page<C: IsA<Widget>> {
    name: &'static str,
    #[builder(via_mutators)]
    options: Vec<C>,
}
impl<C: IsA<Widget>> From<Page<C>> for gtk::Box {
    fn from(value: Page<C>) -> Self {
        page(value.name, value.options)
    }
}

fn page<C: IsA<Widget>>(name: &str, options: Vec<C>) -> gtk::Box {
    let page = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["page"])
        .name(name)
        .spacing(12)
        .build();

    for option in options {
        page.append(&option);
    }

    page
}