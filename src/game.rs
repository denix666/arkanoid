use macroquad::prelude::*;

pub struct Game {
    score: i32,
    lives: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
        }
    }

    pub fn score(&mut self) -> i32 {
        return self.score
    }

    pub fn lives(&mut self) -> i32 {
        return self.lives
    }

    pub fn update_game(&mut self, score: i32, lives: i32) {
        self.score = score;
        self.lives = lives;
    }
}
