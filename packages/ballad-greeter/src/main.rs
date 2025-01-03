pub mod greetd;

use std::{thread::sleep, time::Duration};

use greetd::{GreetdSession, GreeterError, RequestedAction};
use gtk::{Application, glib, prelude::*};

fn main() {
    let app = Application::builder()
        .application_id("com.gavinniederman.ballad-shell")
        .build();

    app.connect_activate(activate);
    app.connect_startup(startup);

    app.run();
}

fn activate(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title(Some("Ballad Shell"));
    window.set_default_size(800, 600);
    window.present();
}

fn startup(app: &Application) {
    let app = app.clone();
    glib::spawn_future_local(async move {
        let greetd_socket = std::env::var("GREETD_SOCK").expect("Cannot find GREETD_SOCK in environment. Is this running as your greeter? Use a program like `greetd-stub` if you are developing this program.");
        let mut session = GreetdSession::new(greetd_socket).await.unwrap();
        loop {
            match session
                .step_statemachine("real", Some("3636"), "firefox")
                .await {
                    Ok(RequestedAction::DisplayMessage(message)) => println!("message: {message}"),
                    Ok(RequestedAction::SendDataFromPrompt { prompt, visible }) => {
                        println!("prompt: {prompt}, visible: {visible}")
                    }
                    Ok(RequestedAction::ExitApplication) => {
                        println!("Exiting");
                        app.quit();
                        break;
                    }
                    Ok(RequestedAction::None) => {},
                    Err(GreeterError::FailedToAuthenticate) => {
                        println!("Failed to authenticate");
                        break;
                    },
                    Err(e) => {
                        println!("Error: {e}");
                        break;
                    }
                }
            sleep(Duration::from_secs(1));
        }
    });
}
