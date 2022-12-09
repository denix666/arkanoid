use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub lives: i32,
    pub lvl_num: i32,
    pub last_enemy_burn_time: f64,
    pub balls_speed: f32,
    pub last_speed_increased: f64,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            lvl_num: 0,
            last_enemy_burn_time: 0.0,
            balls_speed: 120.0,
            last_speed_increased: 0.0,
        }
    }
}
