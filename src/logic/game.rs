use rand::{Rng, seq::SliceRandom};

use crate::WIN_STATE;

//===========================
//
//===========================
#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub round: i8,
    pub now_playing: Players,
    pub who_won: Option<Players>,

    // Board
    pub board: [[Option<Players>; 3]; 3],

    // player 1
    pub player_1: Players,
    pub player_1_win_count: i8,

    // player 2
    pub player_2: Players,
    pub player_2_win_count: i8,
}

//===========================
//
//===========================
impl Game {
    //------------------------------
    //
    //------------------------------
    pub fn play(&mut self, play: PlayParams) -> bool {
        // It is player turn to play?
        if play.val != self.now_playing {
            return false;
        }

        // box must be empty
        let square = self.board[play.y as usize][play.x as usize];
        if square.is_some() {
            return false;
        }

        // player played this
        self.board[play.y as usize][play.x as usize] = Some(play.val);

        // update turn
        self.now_playing = match self.now_playing {
            Players::X => Players::O,
            Players::O => Players::X,
        };

        // did anyone win
        self.check_wins();
        true
    }

    //------------------------------
    //
    //------------------------------
    fn check_wins(&mut self) {
        let mut box_filled_count = 0;
        let mut player_1_win_rate: Vec<String> = vec![];
        let mut player_2_win_rate: Vec<String> = vec![];

        // win logic
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                //----------------------
                // get player
                //----------------------
                let player = self.board[y][x];
                if player == None {
                    continue;
                }

                //==
                box_filled_count += 1;

                //----------------------------
                // Player one check
                //----------------------------
                if player.unwrap() == self.player_1 {
                    player_1_win_rate.push(format!("{y}{x}"));

                    // since it will aways be a square of equal size (optional)
                    // if y != (self.board.len() - 1) && x != (self.board.len() - 1) {
                    //     continue;
                    // }

                    for state in WIN_STATE {
                        let all_exist = state.iter().all(|val| {
                            // For each value in the required list, check if the target list contains it.
                            player_1_win_rate.contains(&val.to_string())
                        });

                        if all_exist {
                            self.who_won = Some(self.player_1);
                            self.update_game_state();
                        }
                    }
                }

                //----------------------------
                // Player two check
                //----------------------------
                if player.unwrap() == self.player_2 {
                    player_2_win_rate.push(format!("{y}{x}"));

                    // since it will aways be a square of equal size
                    // if y != (self.board.len() - 1) && x != (self.board.len() - 1) {
                    //     continue;
                    // }

                    for state in WIN_STATE {
                        let all_exist = state.iter().all(|val| {
                            // For each value in the required list, check if the target list contains it.
                            player_2_win_rate.contains(&val.to_string())
                        });

                        if all_exist {
                            self.who_won = Some(self.player_2);
                            self.update_game_state();
                        }
                    }
                }
            }
        }

        //----------------------------
        // No one won, and all box are filled
        //----------------------------
        if box_filled_count == (self.board.len() * self.board.len()) {
            self.update_game_state();
        }
    }

    //------------------------------
    //
    //------------------------------
    fn update_game_state(&mut self) {
        self.round += 1;
        self.board = [[None, None, None], [None, None, None], [None, None, None]];

        // update score
        match self.who_won {
            Some(player) => {
                if player == self.player_1 {
                    self.player_1_win_count += 1;
                    self.who_won = None;
                }

                if player == self.player_2 {
                    self.player_2_win_count += 1;
                    self.who_won = None;
                }
            }
            None => {}
        }
    }
}

//===========================
//
//===========================
impl Default for Game {
    fn default() -> Self {
        //
        let mut rng = rand::rng();
        let x: usize = rng.random_range(0..=1);

        //
        let mut posible_shape: [Players; 2] = [Players::X, Players::O];
        posible_shape.shuffle(&mut rng);

        Self {
            round: 1,
            who_won: None,
            board: [[None; 3]; 3], // 3x3 grid
            now_playing: posible_shape[x],

            //  player 1
            player_1_win_count: 0,
            player_1: posible_shape[0],

            // player 2
            player_2_win_count: 0,
            player_2: posible_shape[1],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Players {
    X,
    O,
}

pub struct PlayParams {
    pub x: i32,
    pub y: i32,
    pub val: Players,
}
