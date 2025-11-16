#[derive(Debug,Clone)]
pub struct MyBluepringWidget {
    pub box_grid: gtk::Grid,
    pub window: gtk::ApplicationWindow,
    pub player_widget: gtk::Image,
    pub player_win_round: gtk::Label,
    pub bot_widget: gtk::Image,
    pub bot_win_round: gtk::Label,
    pub turn_indicator: gtk::Label,
    pub reset_button: gtk::Button,
    pub indicator_box: gtk::Box,
    pub continue_btn: gtk::Button,
    pub indicator_label: gtk::Label,
}

pub fn my_blueprint_widget(builder: &gtk::Builder) -> MyBluepringWidget {
    let window: gtk::ApplicationWindow = builder
        .object("main_window")
        .expect("Couldn't get window with id \"main_window\"");

    //
    let box_grid: gtk::Grid = builder.object("box_frame").unwrap();

    // player
    let player_widget: gtk::Image = builder.object("player").unwrap();
    let player_win_round: gtk::Label = builder.object("player-win-round").unwrap();

    // Bot Var
    let bot_widget: gtk::Image = builder.object("bot").unwrap();
    let bot_win_round: gtk::Label = builder.object("bot-win-round").unwrap();

    // Turn Indicator
    let turn_indicator: gtk::Label = builder.object("turn-indicator").unwrap();

    // Reset Btn
    let reset_button: gtk::Button = builder.object("reset_btn").unwrap();

    // indicator
    let indicator_box: gtk::Box = builder.object("indicator_box").unwrap();
    // indicator_box.add_css_class("lose");
    let continue_btn: gtk::Button = builder.object("continue_btn").unwrap();
    // continue_btn.add_css_class("lose");
    let indicator_label: gtk::Label = builder.object("indicator_label").unwrap();

    MyBluepringWidget {
        player_widget,
        player_win_round,
        bot_widget,
        bot_win_round,
        turn_indicator,
        reset_button,
        indicator_box,
        continue_btn,
        indicator_label,
        window,
        box_grid,
    }
}
