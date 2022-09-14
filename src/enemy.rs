use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};

const ANIMATION_SPEED: i32 = 8;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    explode_animation_texture: Vec<Texture2D>,
    explode_animation_completed: bool,
    update_interval: i32,
    cur_frame: usize,
    cur_animation_frame: usize,
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
            let path = format!("assets/enemy/enemy_{}_{}.png",enemy_type, i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        let mut explode_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=10 { // Number of sprites in animation
            let path = format!("assets/enemy/enemy_explosion_{}.png", i);
            explode_sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x,
            y,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            cur_animation_frame: 0,
            rect: Rect::new(0.0, 0.0, 0.0,0.0),
            destroyed: false,
            burn_time: get_time(),
            explode_animation_texture: explode_sprites,
            explode_animation_completed: false,
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

    pub fn update(&mut self) {
        self.rect.w = self.texture[self.cur_frame].width();
        self.rect.h = self.texture[self.cur_frame].height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn show_explode_animation(&mut self) {
        let mut diff_in_size_width = 0.0;
        let mut diff_in_size_height = 0.0;
        if !self.explode_animation_completed {
            // center position of animation because of diffrent sprites sizes
            if self.cur_animation_frame > 1 {
                diff_in_size_width = self.explode_animation_texture[self.cur_animation_frame].width() - self.explode_animation_texture[self.cur_animation_frame-1].width();
                diff_in_size_height = self.explode_animation_texture[self.cur_animation_frame].height() - self.explode_animation_texture[self.cur_animation_frame-1].height();
            }
            draw_texture(self.explode_animation_texture[self.cur_animation_frame], self.x - diff_in_size_width, self.y - diff_in_size_height, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED { // Animation speed
                self.update_interval = 0;
                self.cur_animation_frame += 1;
                if self.cur_animation_frame == self.explode_animation_texture.len() {
                    self.cur_animation_frame = 0;
                    self.explode_animation_completed = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            self.update_animation();
            self.update();
            draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
        } else {
            self.show_explode_animation();
        }
    }
}
