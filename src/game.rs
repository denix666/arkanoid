use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub lives: i32,
    pub time_in_seconds: f64,
    pub last_enemy_burn_time: f64,
    pub enemy_amount_now: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            time_in_seconds: 0.0,
            last_enemy_burn_time: 0.0,
            enemy_amount_now: 0,
        }
    }
}
