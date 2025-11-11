mod logic {
    pub mod game;
    pub mod min_max;
}

use std::cell::RefCell;
use std::rc::Rc;

use adw::Application;
use adw::prelude::*;
use gtk;
use gtk::glib;

use crate::logic::game::Game;
use crate::logic::game::PlayParams;
use crate::logic::game::Players;
use crate::logic::min_max::Bot;
use crate::logic::min_max::BotLevel;

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

        let game: Game = Game::default();
        let game_rc: Rc<RefCell<Game>> = Rc::new(RefCell::new(game));
        let box_grid: gtk::Grid = builder.object("box_frame").unwrap();
        let bot: Bot = Bot::new(game_rc.clone(), game_rc.borrow().player_2, BotLevel::Normal);
        let bot_rc: Rc<RefCell<Bot>> = Rc::new(RefCell::new(bot));

        //===========================
        // Get neccessery widget
        //===========================
        let player_widget: gtk::Image = builder.object("player").unwrap();
        let player_win_round: gtk::Label = builder.object("player-win-round").unwrap();

        // Bot Var
        let bot_widget: gtk::Image = builder.object("bot").unwrap();
        let bot_win_round: gtk::Label = builder.object("bot-win-round").unwrap();

        // Turn Indicator
        let turn_indicator: gtk::Label = builder.object("turn-indicator").unwrap();

        // Reset Btn
        let reset_button: gtk::Button = builder.object("reset_btn").unwrap();

        //===========================
        //
        //===========================

        populate_board(
            &bot_rc,
            &game_rc,
            &player_widget,
            &player_win_round,
            &bot_widget,
            &bot_win_round,
            &turn_indicator,
            &box_grid,
        );

        // Bye, time for bed :)

        reset_button.connect_clicked(|_| {
            // game = Game::default();
        });

        //===================
        // Present Window
        //===================
        window.set_application(Some(app));
        window.present();
    });

    app.run();
}

//===========================
// CONSTAINT VARIABLE
//===========================
const X_IMG_PATH: &str = "assets/icons/x.png";
const O_IMG_PATH: &str = "assets/icons/o.png";
const APP_ID: &str = "dev.mc-stephen.tic-tac-toe";

//===========================
//
//===========================
fn get_card(player: &Players) -> Option<&'static str> {
    return match player {
        Players::X => Some(X_IMG_PATH),
        Players::O => Some(O_IMG_PATH),
    };
}

//===========================
// Auto populate grid cells
//===========================
fn populate_board(
    bot_rc: &Rc<RefCell<Bot>>,
    game_rc: &Rc<RefCell<Game>>, // Changed name to 'game_rc' for clarity
    player_widget: &gtk::Image,
    player_win_round: &gtk::Label,
    bot_widget: &gtk::Image,
    bot_win_round: &gtk::Label,
    turn_indicator: &gtk::Label,
    box_grid: &gtk::Grid,
) {
    clear_grid_children(box_grid); // Pass the reference directly

    // Get an immutable borrow once for UI setup
    let bot_state = bot_rc.borrow();
    let game_state = game_rc.borrow();

    // == Update Player/Bot info
    player_win_round.set_label(&game_state.player_1_win_count.to_string());
    player_widget.set_from_file(get_card(&game_state.player_1));

    bot_win_round.set_label(&game_state.player_2_win_count.to_string());
    bot_widget.set_from_file(get_card(&game_state.player_2));

    // == Update Turn Indicator
    turn_indicator.set_label(if game_state.now_playing == game_state.player_1 {
        "Your Turn"
    } else {
        "Bot Turn"
    });

    //--------------------------------
    // Populate Board and assign event
    //--------------------------------
    for y in 0..game_state.board.len() {
        for x in 0..game_state.board[y].len() {
            let image: gtk::Image = gtk::Image::new();
            let x_o_box: gtk::Box = gtk::Box::builder().build();
            let gesture: gtk::GestureClick = gtk::GestureClick::new();

            // ... Box setup ...
            x_o_box.append(&image);
            x_o_box.add_css_class("single");
            x_o_box.add_css_class(&format!("cell-{y}{x}"));
            x_o_box.set_widget_name(&format!("cell-{y}{x}"));

            // ... Image setup ...
            image.set_hexpand(true);
            image.set_halign(gtk::Align::Center);
            image.set_size_request(70, 70);

            // == Attach to grid
            box_grid.attach(&x_o_box, x as i32, y as i32, 1, 1);

            //== Draw existing marks
            if let Some(player_mark) = game_state.board[y][x] {
                // Use the single game_state borrow
                if player_mark == game_state.player_1 {
                    image.set_from_file(get_card(&game_state.player_1));
                }
                if player_mark == game_state.player_2 {
                    image.set_from_file(get_card(&game_state.player_2));
                }
            }

            // --- Capture Variables for Closure (Crucial Fixes Below) ---

            // 1. Clone the necessary handles for the closure to OWN
            let bot_rc_clone: Rc<RefCell<Bot>> = bot_rc.clone();
            let game_rc_clone: Rc<RefCell<Game>> = game_rc.clone();

            //----------------------
            //
            //----------------------
            // if y == (game_state.board.len() - 1) && x == (game_state.board.len() - 1) {
            //     if game_state.now_playing == bot_state.player {
            //         let [y, x] = bot_state.compute();
            //         let delay = time::Duration::from_secs(2);
            //         thread::sleep(delay);
            //         game_rc_clone.borrow_mut().play(PlayParams {
            //             x: x,
            //             y: y,
            //             val: bot_state.player.to_owned(),
            //         });
            //     }
            // }

            // 2. Clone the necessary widgets for the recursive call to accept
            let player_clone = player_widget.clone();
            let player_win_round_clone = player_win_round.clone();
            let bot_clone = bot_widget.clone();
            let bot_win_round_clone = bot_win_round.clone();
            let turn_indicator_clone = turn_indicator.clone();
            let box_grid_clone = box_grid.clone();

            // 3. Prepare simple data for the play call
            let x_val = x as i32; // x and y are i32 implicitly when used in range/indexing, no clone needed
            let y_val = y as i32;
            let player_1_mark = game_state.player_1; // Game state is borrowed once

            //----------------------
            //
            //----------------------
            gesture.connect_released(move |_, _, _, _| {
                // Must use the cloned handles inside the closure

                // 1. Mutate the game state
                let mut game_borrow = game_rc_clone.borrow_mut();
                game_borrow.play(PlayParams {
                    x: x_val, // x_val and y_val are owned by the closure
                    y: y_val,
                    val: player_1_mark, // player_1_mark is owned by the closure
                });
                drop(game_borrow); // Explicitly drop borrow before recursive call

                // 2. Call the function recursively using REFERENCES to the cloned handles
                populate_board(
                    &bot_rc_clone,
                    &game_rc_clone,
                    &player_clone, // Passed by reference to the cloned handle
                    &player_win_round_clone,
                    &bot_clone,
                    &bot_win_round_clone,
                    &turn_indicator_clone,
                    &box_grid_clone,
                );
            });
            x_o_box.add_controller(gesture);
        }
    }

    // Drop the initial borrows before checking bot turn
    drop(bot_state);
    drop(game_state);

    // --- BOT TURN LOGIC MOVED HERE AND MADE NON-BLOCKING ---

    // 1. Check if it's the bot's turn after the board is populated
    if game_rc.borrow().now_playing == bot_rc.borrow().player {
        // Clone all necessary handles for the asynchronous glib closure
        let bot_rc_async = bot_rc.clone();
        let game_rc_async = game_rc.clone();
        let player_widget_async = player_widget.clone();
        let player_win_round_async = player_win_round.clone();
        let bot_widget_async = bot_widget.clone();
        let bot_win_round_async = bot_win_round.clone();
        let turn_indicator_async = turn_indicator.clone();
        let box_grid_async = box_grid.clone();

        // 2. Use glib::timeout_add_seconds_local for a non-blocking 2-second delay
        // This prevents the UI from freezing.
        glib::timeout_add_seconds_local(1, move || {
            // All variables inside this closure are OWNED (cloned)

            // Get the bot's move
            let bot_handle = bot_rc_async.borrow();
            let bot_player = bot_handle.player;
            let [y, x] = bot_handle.compute();
            drop(bot_handle); // Drop immutable borrow

            // Play the move
            let mut game_borrow = game_rc_async.borrow_mut();
            game_borrow.play(PlayParams {
                x: x,
                y: y,
                val: bot_player,
            });
            drop(game_borrow); // Drop mutable borrow before recursive call

            // Recursively call populate_board to update the UI
            populate_board(
                &bot_rc_async,
                &game_rc_async,
                &player_widget_async,
                &player_win_round_async,
                &bot_widget_async,
                &bot_win_round_async,
                &turn_indicator_async,
                &box_grid_async,
            );

            // Return Break to ensure the timer stops after one execution
            glib::ControlFlow::Break
        });
    }
}

//===========================
// Auto populate grid cells
//===========================
fn clear_grid_children(grid: &gtk::Grid) {
    // Start with the first child.
    let mut child = grid.first_child();

    // Loop until the child pointer becomes None (the last one is removed).
    while child.is_some() {
        let current_child = child.unwrap();

        // Store the next sibling *before* removing the current child.
        // This is crucial because removing the child invalidates its sibling relationship.
        let next = current_child.next_sibling();

        // Remove the child from the grid.
        grid.remove(&current_child);

        // Move to the next widget in the list.
        child = next;
    }
}

//===========================
//
//===========================
#[rustfmt::skip]
const WIN_STATE: [[&str;3];8] = [
    // Note: 1st val = y, 2nd val = x

    ["00","01","02"], // Horizontal
    ["10","11","12"], // Horizontal
    ["20","21","22"], // Horizontal

    ["00","10","20"], // Vertical
    ["01","11","21"], // Vertical
    ["02","12","22"], // Vertical

    ["00","11","22"], // Diagonal
    ["02","11","20"], // Diagonal
];
