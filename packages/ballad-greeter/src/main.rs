pub mod greetd;
mod widgets;

use std::{thread::sleep, time::Duration};

use ballad_services::accounts::ACCOUNTS_SERVICE;
use greetd::{GreetdSession, GreeterError, RequestedAction};
use gtk::{Align, Application, Stack, glib, gio, prelude::*};
use widgets::user_select;

fn main() {
    gio::resources_register_include!("icons.gresource").unwrap();

    let greetd_socket = std::env::var("GREETD_SOCK").expect("Cannot find GREETD_SOCK in environment. Is this running as your greeter? Use a program like `greetd-stub` if you are developing this program.");

    let app = Application::builder()
        .application_id("com.gavinniederman.ballad-greeter")
        .build();

    app.connect_activate(activate);
    app.connect_startup(move |app| start_greetd_statemachine(app, greetd_socket.clone()));

    app.run();
}

fn activate(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title(Some("Ballad Greeter"));
    // window.set_fullscreened(true);
    // window.set_decorated(false);

    let login_flow_stack = Stack::builder()
        .hexpand(true)
        .vexpand(true)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    let user_select = user_select(login_flow_stack.clone());
    login_flow_stack.add_titled(&user_select, Some("user-select"), "User Select");
    login_flow_stack.set_visible_child_name("user-select");

    window.set_child(Some(&login_flow_stack));

    window.present();
}

fn start_greetd_statemachine(app: &Application, greetd_socket: String) {
    let app = app.clone();
    // glib::spawn_future_local(async move {
    //     let mut session = GreetdSession::new(greetd_socket).await.unwrap();
    //     loop {
    //         match session
    //             .step_statemachine("real", Some("3636"), "firefox")
    //             .await
    //         {
    //             Ok(RequestedAction::DisplayMessage(message)) => println!("message: {message}"),
    //             Ok(RequestedAction::SendDataFromPrompt { prompt, visible }) => {
    //                 println!("prompt: {prompt}, visible: {visible}")
    //             }
    //             Ok(RequestedAction::ExitApplication) => {
    //                 println!("Exiting");
    //                 app.quit();
    //                 break;
    //             }
    //             Ok(RequestedAction::None) => {}
    //             Err(GreeterError::FailedToAuthenticate) => {
    //                 println!("Failed to authenticate");
    //                 break;
    //             }
    //             Err(e) => {
    //                 println!("Error: {e}");
    //                 break;
    //             }
    //         }
    //         sleep(Duration::from_secs(1));
    //     }
    // });
}
