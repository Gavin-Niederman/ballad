use std::cell::RefCell;

use ballad_services::niri::NIRI_SERVICE;
use ballad_services::niri::NiriService;
use ballad_services::niri::Window;
use ballad_services::niri::Workspace;
use gtk::Align;
use gtk::Button;
use gtk::gdk::Monitor;
use gtk::gio::ListStore;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::closure_local;
use gtk::prelude::*;

use crate::utils::set_class_on_widget;
use crate::widgets::icon::app_icon;

fn window(window: Window) -> Button {
    // Special case for vscode which uses an incorrect app id
    let app_id = window.app_id().map(|app_id| {
        if app_id == "code-url-handler" {
            "code".to_string()
        } else {
            app_id
        }
    });

    let icon = app_icon(app_id.as_deref(), 16);

    let button = Button::builder()
        .css_classes(["window"])
        .name("window")
        .child(&icon)
        .build();
    set_class_on_widget(window.is_focused(), &button, "focused");
    set_class_on_widget(window.is_active(), &button, "active");

    window.connect_closure(
        "changed",
        false,
        closure_local!(
            #[weak]
            button,
            move |window: Window| {
                set_class_on_widget(window.is_focused(), &button, "focused");
                set_class_on_widget(window.is_active(), &button, "active");
            }
        ),
    );

    button.connect_clicked(clone!(
        #[weak]
        window,
        move |_| {
            NIRI_SERVICE.with(|service| {
                smol::block_on(service.focus_window(window.id()));
            })
        }
    ));

    button
}

pub fn windows() -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .name("windows-container")
        .css_classes(["windows"])
        .halign(Align::Center)
        .spacing(4)
        .build();

    NIRI_SERVICE.with(|service| {
        let window_buttons = RefCell::new(Vec::new());

        service.connect_closure(
            "windows-changed",
            false,
            closure_local!(
                #[weak]
                container,
                move |service: NiriService, _: ListStore| {
                    // Remove all children
                    for button in window_buttons.borrow().iter() {
                        container.remove(button);
                    }
                    window_buttons.borrow_mut().clear();

                    for ipc_window in service.windows().iter::<Window>() {
                        let Ok(ipc_window) = ipc_window else {
                            continue;
                        };

                        let button = window(ipc_window);
                        container.append(&button);
                        window_buttons.borrow_mut().push(button);
                    }
                }
            ),
        );
    });

    container
}

fn workspace(workspace: Workspace) -> Button {
    let button = Button::builder()
        .css_classes(["workspace"])
        .name("workspace")
        .hexpand(false)
        .build();
    set_class_on_widget(workspace.is_active(), &button, "active");

    workspace.connect_closure(
        "changed",
        false,
        closure_local!(
            #[weak]
            button,
            move |workspace: Workspace| {
                set_class_on_widget(workspace.is_active(), &button, "active");
            }
        ),
    );

    button.connect_clicked(clone!(
        #[weak]
        workspace,
        move |_| {
            NIRI_SERVICE.with(|service| {
                smol::block_on(service.focus_workspace(workspace.id()));
            })
        }
    ));

    button
}

pub fn workspaces(output: Monitor) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .name("workspaces-container")
        .css_classes(["workspaces"])
        .halign(Align::Center)
        .spacing(6)
        .build();

    NIRI_SERVICE.with(|service| {
        let workspace_buttons = RefCell::new(Vec::new());

        service.connect_closure(
            "workspaces-changed",
            false,
            closure_local!(
                #[weak]
                container,
                move |service: NiriService, _: ListStore| {
                    // Remove all children
                    for button in workspace_buttons.borrow().iter() {
                        container.remove(button);
                    }
                    workspace_buttons.borrow_mut().clear();

                    let mut ipc_workspaces = service
                        .workspaces()
                        .iter::<Workspace>()
                        .filter_map(|workspace| {
                            if let Ok(workspace) = workspace {
                                if workspace.output() == output.connector().map(Into::into) {
                                    Some(workspace)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    ipc_workspaces.sort_by_key(|workspace| workspace.idx());

                    for ipc_workspace in ipc_workspaces {
                        let button = workspace(ipc_workspace);
                        container.append(&button);
                        workspace_buttons.borrow_mut().push(button);
                    }
                }
            ),
        );
    });

    container
}
