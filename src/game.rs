use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub lives: i32,
    pub lvl_num: i32,
    pub last_enemy_burn_time: f64,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            lvl_num: 0,
            last_enemy_burn_time: 0.0,
        }
    }
}
