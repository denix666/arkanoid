use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 700.0;
const PULSATION_SPEED: i32 = 6;

pub enum PaddleType {
    Normal,
    Catch,
    Expand,
    Laser,
}

pub struct Paddle {
    pub x: f32,
    pub y: f32, 
    paddle_catch: Texture2D,
    pub paddle_type: PaddleType,
    pub rect: Rect,
    pub destroyed: bool,
    paddle_wide: Vec<Texture2D>,
    paddle_wide_cur_frame: usize,
    paddle_wide_update_interval: i32,
    paddle_laser: Vec<Texture2D>,
    paddle_laser_cur_frame: usize,
    paddle_laser_update_interval: i32,
    paddle_normal: Vec<Texture2D>,
    paddle_normal_cur_frame: usize,
    paddle_normal_update_interval: i32,
}

impl Paddle {
    pub async fn new() -> Self {
        let mut paddle_wide_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=4 { // Number of sprites in animation
            let path = format!("assets/images/paddle/wide/paddle_wide_{}.png", i);
            paddle_wide_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut paddle_laser_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=4 { // Number of sprites in animation
            let path = format!("assets/images/paddle/laser/paddle_laser_{}.png", i);
            paddle_laser_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut paddle_normal_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=4 { // Number of sprites in animation
            let path = format!("assets/images/paddle/normal/paddle_{}.png", i);
            paddle_normal_sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x: 0.0,
            y: screen_height() - 75.0, 
            paddle_catch: load_texture("assets/images/paddle/paddle_catch.png").await.unwrap(),
            paddle_wide: paddle_wide_sprites,
            paddle_wide_update_interval: 0,
            paddle_wide_cur_frame: 0,
            paddle_laser: paddle_laser_sprites,
            paddle_laser_update_interval: 0,
            paddle_laser_cur_frame: 0,
            paddle_normal: paddle_normal_sprites,
            paddle_normal_update_interval: 0,
            paddle_normal_cur_frame: 0,
            paddle_type: PaddleType::Normal,
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            destroyed: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0.0;

        if is_key_down(KeyCode::Left) {
            x_move -= 1.0;
        }

        if is_key_down(KeyCode::Right) {
            x_move += 1.0;
        }

        self.x += x_move * dt * PLAYER_SPEED;

        self.x = mouse_position().0;

        if self.x < crate::resources::FRAME_INDENT {
            self.x = crate::resources::FRAME_INDENT;
        }

        match self.paddle_type {
            PaddleType::Normal => {
                if self.x > screen_width() - crate::resources::FRAME_INDENT - self.paddle_normal[0].width() {
                    self.x = screen_width() - crate::resources::FRAME_INDENT - self.paddle_normal[0].width();
                }
                self.rect.w = self.paddle_normal[0].width();
                self.rect.h = self.paddle_normal[0].height();
            },
            PaddleType::Catch => {
                if self.x > screen_width() - crate::resources::FRAME_INDENT - self.paddle_catch.width() {
                    self.x = screen_width() - crate::resources::FRAME_INDENT - self.paddle_catch.width();
                }
                self.rect.w = self.paddle_catch.width();
                self.rect.h = self.paddle_catch.height();
            },
            PaddleType::Expand => {
                if self.x > screen_width() - crate::resources::FRAME_INDENT - self.paddle_wide[0].width() {
                    self.x = screen_width() - crate::resources::FRAME_INDENT - self.paddle_wide[0].width();
                }
                self.rect.w = self.paddle_wide[0].width();
                self.rect.h = self.paddle_wide[0].height();
            },
            PaddleType::Laser => {
                if self.x > screen_width() - crate::resources::FRAME_INDENT - self.paddle_laser[0].width() {
                    self.x = screen_width() - crate::resources::FRAME_INDENT - self.paddle_laser[0].width();
                }
                self.rect.w = self.paddle_laser[0].width();
                self.rect.h = self.paddle_laser[0].height();
            },
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn center_x(&mut self) -> f32 {
        match self.paddle_type {
            PaddleType::Normal => {
                self.x + self.paddle_normal[0].width() / 2.0
            },
            PaddleType::Catch => {
                self.x + self.paddle_catch.width() / 2.0
            },
            PaddleType::Expand => {
                self.x + self.paddle_wide[0].width() / 2.0
            },
            PaddleType::Laser => {
                self.x + self.paddle_laser[0].width() / 2.0
            },
        }
    }

    pub fn width(&mut self) -> f32 {
        match self.paddle_type {
            PaddleType::Normal => {
                self.paddle_normal[0].width()
            },
            PaddleType::Catch => {
                self.paddle_catch.width()
            },
            PaddleType::Expand => {
                self.paddle_wide[0].width()
            },
            PaddleType::Laser => {
                self.paddle_laser[0].width()
            },
        }
    }

    pub fn paddle_wide_draw(&mut self) {
        draw_texture(self.paddle_wide[self.paddle_wide_cur_frame], self.x, self.y, WHITE);
        self.paddle_wide_update_interval += 1;
        if self.paddle_wide_update_interval > PULSATION_SPEED { // Animation speed for pulsation
            self.paddle_wide_update_interval = 0;
            self.paddle_wide_cur_frame += 1;
            if self.paddle_wide_cur_frame == self.paddle_wide.len() {
                self.paddle_wide_cur_frame = 0;
            }
        }
    }

    pub fn paddle_normal_draw(&mut self) {
        draw_texture(self.paddle_normal[self.paddle_normal_cur_frame], self.x, self.y, WHITE);
        self.paddle_normal_update_interval += 1;
        if self.paddle_normal_update_interval > PULSATION_SPEED { // Animation speed for pulsation
            self.paddle_normal_update_interval = 0;
            self.paddle_normal_cur_frame += 1;
            if self.paddle_normal_cur_frame == self.paddle_normal.len() {
                self.paddle_normal_cur_frame = 0;
            }
        }
    }

    pub fn paddle_laser_draw(&mut self) {
        draw_texture(self.paddle_laser[self.paddle_laser_cur_frame], self.x, self.y, WHITE);
        self.paddle_laser_update_interval += 1;
        if self.paddle_laser_update_interval > PULSATION_SPEED { // Animation speed for pulsation
            self.paddle_laser_update_interval = 0;
            self.paddle_laser_cur_frame += 1;
            if self.paddle_laser_cur_frame == self.paddle_laser.len() {
                self.paddle_laser_cur_frame = 0;
            }
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            match self.paddle_type {
                PaddleType::Normal => {
                    self.paddle_normal_draw();
                },
                PaddleType::Catch => {
                    draw_texture(self.paddle_catch, self.x, self.y, WHITE);
                },
                PaddleType::Expand => {
                    self.paddle_wide_draw();
                },
                PaddleType::Laser => {
                    self.paddle_laser_draw();
                },
            }
        }
    }
}
