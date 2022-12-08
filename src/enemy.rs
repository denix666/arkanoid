use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};

const ANIMATION_SPEED: i32 = 8;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub destroyed: bool,
    pub burn_time: f64,
}

impl Enemy {
    pub async fn new(x:f32, y:f32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        let enemy_type: &str = match rand::thread_rng().gen_range(0..=3) { 
            0 => "cone",
            1 => "cube",
            2 => "molecule",
            _ => "pyramid",
        };

        for i in 1..=25 {
            let path = format!("assets/images/enemy/enemy_{}_{}.png",enemy_type, i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x,
            y,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 0.0,0.0),
            destroyed: false,
            burn_time: get_time(),
        }
    }

    pub fn update_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.texture.len() {
                self.cur_frame = 0;
            }
        }
    }

    pub fn update(&mut self, paddle_x: f32, paddle_y: f32) {
        if self.y < paddle_y {
            self.y += 1.0;
        }
        if self.x < paddle_x {
            self.x += 1.0;
        } else 
        if self.x > paddle_x {
            self.x -= 1.0;
        }

        self.rect.w = self.texture[self.cur_frame].width();
        self.rect.h = self.texture[self.cur_frame].height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        self.update_animation();
        draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
    }
}
