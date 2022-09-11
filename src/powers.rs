use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};

const POWER_SPEED: f32 = 90.0;
const NUM_OF_FRAMES: usize = 8;
const ANIMATION_SPEED: i32 = 4;

pub struct Power {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    cur_frame: usize,
    update_interval: i32,
    pub actual: bool,
    pub rect: Rect,
    pub kind: String,
}

impl Power {
    pub async fn new(x:f32, y:f32) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        let powerup_type: &str = match rand::thread_rng().gen_range(0..=6) { 
            0 => "laser",
            1 => "life",
            2 => "slow",
            3 => "catch",
            4 => "duplicate",
            5 => "warp",
            _ => "expand",
        };

        for i in 1..=NUM_OF_FRAMES {
            let path = format!("assets/powers/powerup_{}_{}.png",powerup_type, i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x,
            y,
            texture: sprites,
            cur_frame: 0,
            update_interval: 0,
            actual: true,
            rect: Rect::new(x, y, 38.0, 19.0),
            kind: powerup_type.to_string(),
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

    pub fn update_position(&mut self, dt: f32) {
        self.y += dt * POWER_SPEED;
    }

    pub fn update(&mut self) {
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        if self.actual {
            self.update_position(get_frame_time());
            self.update_animation();
            self.update();
            draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
        }
    }
}