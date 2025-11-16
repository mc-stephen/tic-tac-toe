use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::logic::game::{Game, Players};

#[derive(Debug, Clone)]
pub struct Bot {
    pub player: Players,
    pub bot_level: BotLevel,
}

impl Bot {
    //==================================
    // Initialize new bot with dynamic data
    //==================================
    pub fn new(player: &Players, level: BotLevel) -> Bot {
        Self {
            bot_level: level,
            player: player.to_owned(),
        }
    }

    //====================================
    // Return the y and x box axis to click
    //====================================
    pub fn compute(&self, game: &Rc<RefCell<Game>>) -> [i32; 2] {
        let now_playing: Players = self.player;

        match self.bot_level {
            BotLevel::Normal => self.easy_level(game),
            BotLevel::Hard => self.normal_hard_level(now_playing, game.borrow().board.clone()),
        }
    }

    //====================================
    // Easy (Random Selector)
    //====================================
    fn easy_level(&self, game: &Rc<RefCell<Game>>) -> [i32; 2] {
        let mut rng = rand::rng();
        let y: i32 = rng.random_range(0..3) as i32;
        let x: i32 = rng.random_range(0..3) as i32;

        // if game.borrow().board[y as usize][x as usize].is_some() {
        //     return self.easy_level(game);
        // }

        [y, x]
        // [0, 0]
    }

    // fn easy_level(&self, game: Rc<RefCell<Game>>) -> [i32; 2] {
    //     let mut rng = rand::rng();
    //     let game_ref = game.borrow();

    //     loop {
    //         let y: i32 = rng.random_range(0..3);
    //         let x: i32 = rng.random_range(0..3);

    //         if game_ref.board[y as usize][x as usize].is_none() {
    //             return [y, x];
    //         }
    //     }
    // }

    //====================================
    // Normal / Hard Level Logic
    //====================================
    fn normal_hard_level(
        &self,
        now_playing: Players,
        board: [[Option<Players>; 3]; 3],
    ) -> [i32; 2] {
        let end_state = self.min_max(now_playing, board);

        //
        [0, 0]
    }

    //====================================
    // Easy (Random Selector)
    //====================================
    fn min_max(
        &self,
        now_playing: Players,
        mut board: [[Option<Players>; 3]; 3],
    ) -> [[Option<Players>; 3]; 3] {
        let mut yx: [usize; 2] = [0, 0];

        // loop through the current board state
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                // find empty cells
                if let None = board[y][x] {
                    // save cell axis (in case this is a win)
                    yx = [y, x];

                    // put now_playing piece in cell
                    board[y][x] = Some(now_playing);

                    // update now playing
                    let now_playing = match now_playing {
                        Players::X => Players::O,
                        Players::O => Players::X,
                    };

                    // check if is a win (end if win found)
                    break;

                    // check if game has ended

                    // loop through the new board state(s)
                    self.min_max(now_playing, board);
                }
            }
        }

        board

        //~~~~~~~~~~~~~~~ Steps ~~~~~~~~~~~~~~~~~~~
        // clone board (test board)
        // loop through the current board state
        // find empty cells
        // put now_playing piece in cell (bot)
        // update now playing
        //---- loop through the new board state(s) (cloned)
        //---- find empty cells
        //---- put now_playing piece in cell (enemy)
        //---- update now playing
        //--------- repeat till game ends or selected depth
        //-------------- (pick end state that favours bot)
        //-------------- do this by comparing the all end state of the bot to posible "WIN_STATE"
        //-------------- and getting the best one, or rather the one, where the enemy doesn't win
        //------------------- send [y,x] of the picked board state (best one)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BotLevel {
    Normal,
    Hard,
}
