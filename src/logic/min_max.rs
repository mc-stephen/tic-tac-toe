use crate::logic::game::Players;

#[derive(Debug,Clone, Copy)]
pub struct MinMax {
    bot_depth: i32,
    pub bot: Players,
    pub bot_level: BotLevel,
}

impl MinMax {
    //========================
    //
    //========================
    pub fn new(&mut self, player: Players, level: BotLevel) {
        self.bot = player;
        self.bot_level = level;
        self.bot_depth = match level {
            BotLevel::Easy => 0,
            BotLevel::Normal => 5,
            BotLevel::Hard => 10,
        };
    }

    //========================
    //
    //========================
    pub fn play() {}
}

#[derive(Debug,Clone, Copy)]
pub enum BotLevel {
    Easy,
    Normal,
    Hard,
}
