// use gtk::prelude::*;
// use gtk::{Application, ApplicationWindow, Builder, Button};

// const APP_ID: &str = "dev.mc-stephen.tic-tac-toe";

// fn main() {
//     let app = Application::builder().application_id(APP_ID).build();

//     app.connect_activate(|app| {
//         let builder = Builder::from_file("target/ui/window.ui");
//         // find window and button by ID
//         let window: ApplicationWindow = builder
//             .object("main_window")
//             .expect("main_window not found");
//         let button: Button = builder.object("button").expect("button not found");

//         // connect safely in Rust
//         button.connect_clicked(move |_| {
//             println!("Button clicked (manual connect)!");
//         });

//         window.set_application(Some(app));
//         window.present();
//     });

//     app.run();
// }

use adw::Application;
use adw::prelude::*;
use gtk;

const APP_ID: &str = "dev.mc-stephen.tic-tac-toe";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        // ✅ Load the CSS file
        let provider = gtk::CssProvider::new();
        provider.load_from_path("src/ui/style.css");

        // ✅ Apply globally
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Load ui file
        let builder = gtk::Builder::from_file("target/ui/window.ui");
        let window: gtk::ApplicationWindow = builder
            .object("main_window")
            .expect("Couldn't get window with id \"main_window\"");

        let box_grid: gtk::Grid = builder.object("box_frame").unwrap();

        // Auto populate grid cells
        for x in 0..3 {
            for y in 0..3 {
                let gesture = gtk::GestureClick::new();
                let x_o_box: gtk::Box = gtk::Box::builder()
                    .name(format!("cell-{x}{y}"))
                    .css_classes(["single", &format!("cell-{x}{y}")])
                    .build();

                box_grid.attach(&x_o_box, y, x, 1, 1);
                gesture.connect_released(|_, _, _, _| {
                    x_o_box_fn();
                });
                x_o_box.add_controller(gesture);
            }
        }

        // Player Var
        // let player = builder.object().unwrap();
        // let player_win_round = builder.object().unwrap();

        // Bot Var
        // let bot = builder.object().unwrap();
        // let bot_win_round = builder.object().unwrap();

        // Turn Indicator
        // let turn_indicator = builder.object().unwrap();

        // Grid Cell
        // let cell_01 = builder.object().unwrap();

        // Reset Btn
        let button: gtk::Button = builder.object("reset_btn").unwrap();
        button.connect_clicked(|_| {
            println!("Clicked from Blueprint UI!");
        });

        // Present Window
        window.set_application(Some(app));
        window.present();
    });

    app.run();
}

//===========================
//
//===========================
fn x_o_box_fn() {
    println!("Box clicked!");
}
