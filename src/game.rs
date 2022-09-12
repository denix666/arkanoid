use macroquad::prelude::*;

pub struct Game {
    score: i32,
    lives: i32,
    time_in_seconds: f64,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            time_in_seconds: 0.0,
        }
    }

    pub fn score(&mut self) -> i32 {
        return self.score
    }

    pub fn time(&mut self) -> f64 {
        return self.time_in_seconds
    }

    pub fn set_time(&mut self, time: f64) {
        self.time_in_seconds = time;
    }

    pub fn lives(&mut self) -> i32 {
        return self.lives
    }

    pub fn update_game(&mut self, score: i32, lives: i32) {
        self.score = score;
        self.lives = lives;
    }
}
