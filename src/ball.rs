use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};

use crate::resources::Resources;

pub enum HorizontalDir {
    Left,
    Right,
}

pub enum VerticalDir {
    Up,
    Down,
}

pub struct Ball {
    texture: Texture2D,
    pub x: f32,
    pub y: f32,
    pub ball_step_move_x: f32,
    pub ball_step_move_y: f32,
    pub horizontal_dir: HorizontalDir,
    pub vertical_dir: VerticalDir,
    pub rect: Rect,
    pub released: bool,
    pub last_ball_time: f64,
    pub idle_time: f64,
    pub speed: f32,
    pub destroyed: bool,
}

impl Ball {
    pub async fn new(x: f32,y: f32) -> Self {
        Self {
            texture: load_texture("assets/images/ball.png").await.unwrap(),
            x,
            y,
            ball_step_move_x: 1.0,
            ball_step_move_y: 3.0,
            horizontal_dir: HorizontalDir::Right,
            vertical_dir: VerticalDir::Up,
            rect: Rect::new(x+4.0, y+4.0, 8.0, 8.0),
            released: false,
            last_ball_time: 0.0,
            idle_time: 4.0,
            speed: 120.0,
            destroyed: false,
        }
    }

    pub fn update(&mut self, dt: f32, resources: &Resources) {
        match self.horizontal_dir {
            HorizontalDir::Left => {
                self.x -= self.ball_step_move_x * dt * self.speed;
                if self.x < crate::resources::FRAME_INDENT {
                    self.horizontal_dir = HorizontalDir::Right;
                    play_sound(resources.brick_hit, PlaySoundParams {
                        looped: false,
                        volume: 0.3,
                    });
                }
            },
            HorizontalDir::Right => {
                self.x += self.ball_step_move_x * dt * self.speed;
                if self.x > screen_width() - crate::resources::FRAME_INDENT - self.width() {
                    self.horizontal_dir = HorizontalDir::Left;
                    play_sound(resources.brick_hit, PlaySoundParams {
                        looped: false,
                        volume: 0.3,
                    });
                }
            },
        }

        match self.vertical_dir {
            VerticalDir::Up => {
                self.y -= self.ball_step_move_y * dt * self.speed;
                if self.y < 22.0 {
                    self.vertical_dir = VerticalDir::Down;
                    play_sound(resources.brick_hit, PlaySoundParams {
                        looped: false,
                        volume: 0.3,
                    });
                }
            },
            VerticalDir::Down => {
                self.y += self.ball_step_move_y * dt * self.speed;
                if self.y > screen_height() - 75.0 {
                    self.destroyed = true;
                    play_sound(resources.brick_hit, PlaySoundParams {
                        looped: false,
                        volume: 0.3,
                    });
                }
            },
        }
        self.rect.x = self.x+4.0;
        self.rect.y = self.y+4.0;
    }

    pub fn center_x(&mut self) -> f32 {
        self.x + self.texture.width() / 2.0
    }

    pub fn width(&mut self) -> f32 {
        self.texture.width()
    }

    // pub fn ball_height(&mut self) -> f32 {
    //     self.texture.height()
    // }

    pub fn draw(&self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
